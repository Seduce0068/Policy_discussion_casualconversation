//! Find OKF concept documents by frontmatter metadata.
//!
//! Usage:
//!
//! ```text
//! rust-script scripts/okf-find.rs [OPTIONS]
//! ```
//!
//! The script scans the content directories (`adr`, `chats`, `notes`,
//! `reports`), reads the YAML frontmatter of each concept document, and
//! prints the entries that match all given filters. Reserved files
//! (`index.md`, `log.md`) and hidden paths are skipped.
//!
//! Output is tab-separated: `path`, `type`, `status`, `title`,
//! `description`, `tags`. Use `--paths-only` to print only paths.
//!
//! Exit codes: 0 when at least one document matches, 1 when none match,
//! 2 on usage or I/O errors.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const CONTENT_DIRS: [&str; 4] = ["adr", "chats", "notes", "reports"];

#[derive(Debug, Default)]
struct Options {
    doc_type: Option<String>,
    tags: Vec<String>,
    status: Option<String>,
    text: Option<String>,
    dir: Option<String>,
    root: PathBuf,
    paths_only: bool,
    help: bool,
}

#[derive(Debug)]
struct Entry {
    path: String,
    title: String,
    description: String,
    doc_type: String,
    status: String,
    tags: Vec<String>,
}

#[derive(Debug, Default)]
struct Metadata {
    fields: Vec<(String, String)>,
    tags: Vec<String>,
}

impl Metadata {
    fn field(&self, key: &str) -> Option<&str> {
        self.fields
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.as_str())
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(err) => {
            eprintln!("okf-find: {err}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<bool, String> {
    let options = parse_args()?;
    if options.help {
        print_help();
        return Ok(true);
    }

    let dirs: Vec<&str> = match options.dir.as_deref() {
        Some(dir) => vec![dir],
        None => CONTENT_DIRS.to_vec(),
    };

    let mut entries = Vec::new();
    let mut found_dir = false;
    for dir in dirs {
        let path = options.root.join(dir);
        if !path.is_dir() {
            continue;
        }
        found_dir = true;
        walk(&path, dir, &mut entries)?;
    }

    if !found_dir {
        return Err(format!(
            "no content directories under {}",
            options.root.display()
        ));
    }

    let matched: Vec<&Entry> = entries
        .iter()
        .filter(|entry| options.matches(entry))
        .collect();

    if matched.is_empty() {
        return Ok(false);
    }

    for entry in matched {
        if options.paths_only {
            println!("{}", entry.path);
        } else {
            println!(
                "{}\t{}\t{}\t{}\t{}\t{}",
                clean(&entry.path),
                clean(&entry.doc_type),
                clean(&entry.status),
                clean(&entry.title),
                clean(&entry.description),
                clean(&entry.tags.join(","))
            );
        }
    }
    Ok(true)
}

fn parse_args() -> Result<Options, String> {
    let mut options = Options {
        root: PathBuf::from("."),
        ..Options::default()
    };

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--type" => options.doc_type = Some(next_value(&mut args, "--type")?),
            "--tag" => options.tags.push(next_value(&mut args, "--tag")?),
            "--status" => options.status = Some(next_value(&mut args, "--status")?),
            "--text" => options.text = Some(next_value(&mut args, "--text")?),
            "--dir" => options.dir = Some(next_value(&mut args, "--dir")?),
            "--root" => options.root = PathBuf::from(next_value(&mut args, "--root")?),
            "-l" | "--paths-only" => options.paths_only = true,
            "-h" | "--help" => options.help = true,
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    if let Some(dir) = options.dir.as_deref() {
        if !CONTENT_DIRS.contains(&dir) {
            return Err(format!(
                "unknown --dir: {dir} (expected one of: {})",
                CONTENT_DIRS.join(", ")
            ));
        }
    }

    Ok(options)
}

fn next_value(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn print_help() {
    println!("Usage: rust-script scripts/okf-find.rs [OPTIONS]");
    println!();
    println!("Find OKF concept documents by frontmatter metadata.");
    println!();
    println!("Options:");
    println!("  --type <value>     Exact match on `type` (case-insensitive)");
    println!("  --tag <value>      Tag filter; repeat for AND");
    println!("  --status <value>   Exact match on `status` (case-insensitive)");
    println!("  --text <value>     Substring in path, type, status, title, description, tags");
    println!("  --dir <name>       Limit to one content directory");
    println!("  --root <path>      Repository root. Defaults to the current directory.");
    println!("  -l, --paths-only   Print only file paths");
    println!("  -h, --help         Show this help");
    println!();
    println!("Output is TSV: path, type, status, title, description, tags.");
    println!("Exit codes: 0 = matches, 1 = no matches, 2 = error.");
}

fn walk(dir: &Path, prefix: &str, out: &mut Vec<Entry>) -> Result<(), String> {
    let mut items = fs::read_dir(dir)
        .map_err(|err| format!("{}: {err}", dir.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| format!("{}: {err}", dir.display()))?;
    items.sort_by_key(|item| item.file_name());

    for item in items {
        let path = item.path();
        let name = item.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }

        let file_type = item
            .file_type()
            .map_err(|err| format!("{}: {err}", path.display()))?;
        if file_type.is_dir() {
            walk(&path, &format!("{prefix}/{name}"), out)?;
            continue;
        }
        if !file_type.is_file() || path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        if name == "index.md" || name == "log.md" {
            continue;
        }

        let text = fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
        let Some(metadata) = parse_frontmatter(&text) else {
            eprintln!("okf-find: {}: missing YAML frontmatter", path.display());
            continue;
        };

        let title = metadata
            .field("title")
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| name.trim_end_matches(".md").to_string());

        out.push(Entry {
            path: format!("{prefix}/{name}"),
            title: title.to_string(),
            description: metadata
                .field("description")
                .unwrap_or_default()
                .to_string(),
            doc_type: metadata.field("type").unwrap_or_default().to_string(),
            status: metadata.field("status").unwrap_or_default().to_string(),
            tags: metadata.tags,
        });
    }
    Ok(())
}

fn parse_frontmatter(text: &str) -> Option<Metadata> {
    let mut lines = text.lines();
    if lines.next()?.trim() != "---" {
        return None;
    }

    let mut block = Vec::new();
    let mut closed = false;
    for line in lines {
        if line.trim_end() == "---" {
            closed = true;
            break;
        }
        block.push(line);
    }
    if !closed {
        return None;
    }
    parse_block(&block)
}

fn parse_block(lines: &[&str]) -> Option<Metadata> {
    let mut metadata = Metadata::default();
    let mut pending: Option<(String, String, char)> = None;
    let mut tags_mode = false;

    for line in lines {
        if let Some((key, mut value, quote)) = pending.take() {
            value.push(' ');
            value.push_str(line.trim());
            if value.trim_end().ends_with(quote) {
                metadata
                    .fields
                    .push((key, finish_quoted(value.trim(), quote)));
            } else {
                pending = Some((key, value, quote));
            }
            continue;
        }

        if tags_mode {
            let trimmed = line.trim();
            if let Some(item) = trimmed.strip_prefix('-') {
                let item = item.trim();
                if !item.is_empty() {
                    metadata.tags.push(unquote(item));
                }
                continue;
            }
            tags_mode = false;
        }

        if line.starts_with(' ') || line.starts_with('\t') {
            continue;
        }
        let trimmed = line.trim_end();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once(':') else {
            continue;
        };
        let key = key.trim();
        if key.is_empty() {
            continue;
        }
        let value = value.trim();

        if key == "tags" {
            if value.is_empty() {
                tags_mode = true;
            } else if value.starts_with('[') {
                metadata.tags.extend(parse_flow_list(value));
            } else {
                metadata.tags.push(unquote(value));
            }
            continue;
        }

        match value.chars().next() {
            Some(quote @ ('"' | '\'')) if !(value.len() >= 2 && value.ends_with(quote)) => {
                pending = Some((key.to_string(), value.to_string(), quote));
            }
            Some(quote @ ('"' | '\'')) => {
                metadata
                    .fields
                    .push((key.to_string(), finish_quoted(value, quote)));
            }
            _ => metadata.fields.push((key.to_string(), value.to_string())),
        }
    }

    if pending.is_some() {
        return None;
    }
    Some(metadata)
}

fn parse_flow_list(value: &str) -> Vec<String> {
    let inner = value.trim();
    let inner = inner.strip_prefix('[').unwrap_or(inner);
    let inner = inner.strip_suffix(']').unwrap_or(inner);

    let mut items = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;

    for ch in inner.chars() {
        match quote {
            Some(q) => {
                if ch == q {
                    quote = None;
                } else {
                    current.push(ch);
                }
            }
            None => match ch {
                '"' | '\'' => quote = Some(ch),
                ',' => {
                    let item = current.trim();
                    if !item.is_empty() {
                        items.push(item.to_string());
                    }
                    current.clear();
                }
                _ => current.push(ch),
            },
        }
    }

    let item = current.trim();
    if !item.is_empty() {
        items.push(item.to_string());
    }
    items
}

fn finish_quoted(value: &str, quote: char) -> String {
    let value = value.trim();
    let start = usize::from(value.starts_with(quote));
    let end = value
        .len()
        .saturating_sub(usize::from(value.ends_with(quote)));
    value[start..end].to_string()
}

fn unquote(value: &str) -> String {
    let value = value.trim();
    let mut chars = value.chars();
    match (chars.next(), value.chars().last()) {
        (Some(first), Some(last))
            if (first == '"' || first == '\'') && first == last && value.len() >= 2 =>
        {
            value[1..value.len() - 1].to_string()
        }
        _ => value.to_string(),
    }
}

impl Options {
    fn matches(&self, entry: &Entry) -> bool {
        if let Some(doc_type) = &self.doc_type {
            if !eq_fold(&entry.doc_type, doc_type) {
                return false;
            }
        }
        if let Some(status) = &self.status {
            if !eq_fold(&entry.status, status) {
                return false;
            }
        }
        for tag in &self.tags {
            if !entry.tags.iter().any(|value| eq_fold(value, tag)) {
                return false;
            }
        }
        if let Some(text) = &self.text {
            let needle = text.to_lowercase();
            let haystack = [
                entry.path.as_str(),
                entry.doc_type.as_str(),
                entry.status.as_str(),
                entry.title.as_str(),
                entry.description.as_str(),
                &entry.tags.join(" "),
            ]
            .join(" ");
            if !haystack.to_lowercase().contains(&needle) {
                return false;
            }
        }
        true
    }
}

fn eq_fold(left: &str, right: &str) -> bool {
    left.to_lowercase() == right.to_lowercase()
}

fn clean(value: &str) -> String {
    value
        .replace('\t', " ")
        .replace('\n', " ")
        .replace('\r', " ")
}
