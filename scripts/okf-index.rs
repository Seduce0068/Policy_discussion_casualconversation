//! Regenerate the generated block in OKF `index.md` files.
//!
//! Usage:
//!
//! ```text
//! rust-script scripts/okf-index.rs [--check] [--root <path>]
//! ```
//!
//! For each content directory, the script reads the YAML frontmatter of
//! direct child concept documents and rewrites the block between
//! `<!-- okf-index:begin -->` and `<!-- okf-index:end -->` in that
//! directory's `index.md`.
//!
//! The generated block is a markdown table of `title`, `status`, and
//! `description`, sorted by the date prefix of the filename (newest first)
//! and then by filename. Reserved files (`index.md`, `log.md`) are skipped.
//!
//! `--check` writes nothing and exits with status 1 when any index is out
//! of date. This is the mode to use in CI.

use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const BEGIN: &str = "<!-- okf-index:begin -->";
const END: &str = "<!-- okf-index:end -->";
const CONTENT_DIRS: [&str; 4] = ["adr", "chats", "notes", "reports"];

#[derive(Debug)]
struct Entry {
    file: String,
    title: String,
    description: String,
    status: String,
    sort_key: String,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("okf-index: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let mut check = false;
    let mut root = PathBuf::from(".");

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" => check = true,
            "--root" => {
                let value = args.next().ok_or("--root requires a path")?;
                root = PathBuf::from(value);
            }
            "-h" | "--help" => {
                print_help();
                return Ok(ExitCode::SUCCESS);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    let mut changed = Vec::new();
    for dir in CONTENT_DIRS {
        let dir_path = root.join(dir);
        if !dir_path.is_dir() {
            continue;
        }

        let path = dir_path.join("index.md");
        let entries = collect_entries(&dir_path)?;
        let body = render(&entries);
        let current = match fs::read_to_string(&path) {
            Ok(value) => value,
            Err(err) if err.kind() == ErrorKind::NotFound => default_index(dir),
            Err(err) => return Err(format!("{}: {err}", path.display())),
        };
        let next =
            replace_block(&current, &body).map_err(|err| format!("{}: {err}", path.display()))?;

        if next != current {
            changed.push(path.clone());
            if !check {
                fs::write(&path, next).map_err(|err| format!("{}: {err}", path.display()))?;
            }
        }
    }

    if check && !changed.is_empty() {
        for path in &changed {
            println!("out of date: {}", path.display());
        }
        return Ok(ExitCode::from(1));
    }

    for path in &changed {
        println!("updated: {}", path.display());
    }
    Ok(ExitCode::SUCCESS)
}

fn print_help() {
    println!("Usage: rust-script scripts/okf-index.rs [--check] [--root <path>]");
    println!();
    println!("Rewrite the okf-index block in each content directory's index.md.");
    println!("  --check  Write nothing and exit 1 when an index is out of date.");
    println!("  --root   Repository root. Defaults to the current directory.");
}

fn default_index(dir: &str) -> String {
    format!("# {dir} 一覧\n\n## 文書一覧\n\n{BEGIN}\n{END}\n")
}

fn collect_entries(dir: &Path) -> Result<Vec<Entry>, String> {
    let mut entries = Vec::new();
    let read_dir = fs::read_dir(dir).map_err(|err| format!("{}: {err}", dir.display()))?;

    for item in read_dir {
        let path = item
            .map_err(|err| format!("{}: {err}", dir.display()))?
            .path();
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }
        let file = match path.file_name().and_then(|value| value.to_str()) {
            Some(value) => value.to_string(),
            None => continue,
        };
        if file == "index.md" || file == "log.md" {
            continue;
        }

        let text = fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
        let fields = parse_frontmatter(&text)
            .ok_or_else(|| format!("{}: missing YAML frontmatter", path.display()))?;
        let title = field(&fields, "title")
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| file.trim_end_matches(".md").to_string());
        let description = field(&fields, "description").unwrap_or_default();
        let status = field(&fields, "status").unwrap_or_default();
        let sort_key = filename_date(&file).unwrap_or_default();

        entries.push(Entry {
            file,
            title,
            description,
            status,
            sort_key,
        });
    }

    entries.sort_by(|left, right| {
        right
            .sort_key
            .cmp(&left.sort_key)
            .then(left.file.cmp(&right.file))
    });
    Ok(entries)
}

fn parse_frontmatter(text: &str) -> Option<Vec<(String, String)>> {
    let mut lines = text.lines();
    if lines.next()?.trim() != "---" {
        return None;
    }

    let mut fields = Vec::new();
    let mut pending: Option<(String, String, char)> = None;

    for line in lines {
        let trimmed = line.trim_end();
        if trimmed == "---" {
            return if pending.is_some() {
                None
            } else {
                Some(fields)
            };
        }

        if let Some((key, mut value, quote)) = pending.take() {
            value.push(' ');
            value.push_str(trimmed.trim());
            if value.ends_with(quote) {
                fields.push((key, finish_quoted(&value, quote)));
            } else {
                pending = Some((key, value, quote));
            }
            continue;
        }

        if line.starts_with(' ') || line.starts_with('\t') {
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
        match value.chars().next() {
            Some(quote @ ('"' | '\'')) if !(value.len() >= 2 && value.ends_with(quote)) => {
                pending = Some((key.to_string(), value.to_string(), quote));
            }
            Some(quote @ ('"' | '\'')) => {
                fields.push((key.to_string(), finish_quoted(value, quote)));
            }
            _ => fields.push((key.to_string(), value.to_string())),
        }
    }
    None
}

fn finish_quoted(value: &str, quote: char) -> String {
    let start = usize::from(value.starts_with(quote));
    let end = value
        .len()
        .saturating_sub(usize::from(value.ends_with(quote)));
    value[start..end].to_string()
}

fn field(fields: &[(String, String)], key: &str) -> Option<String> {
    fields
        .iter()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value.clone())
}

fn filename_date(file: &str) -> Option<String> {
    if file.len() < 10 {
        return None;
    }
    let (date, rest) = file.split_at(10);
    let is_date = date.as_bytes().iter().enumerate().all(|(index, byte)| {
        if index == 4 || index == 7 {
            *byte == b'-'
        } else {
            byte.is_ascii_digit()
        }
    });
    if is_date && (rest.starts_with('-') || rest == ".md") {
        Some(date.to_string())
    } else {
        None
    }
}

fn render(entries: &[Entry]) -> String {
    if entries.is_empty() {
        return "- 記録なし。\n".to_string();
    }

    let mut out = String::from("| 文書 | 状態 | 概要 |\n| --- | --- | --- |\n");
    for entry in entries {
        let status = if entry.status.is_empty() {
            "-".to_string()
        } else {
            escape(&entry.status)
        };
        let description = if entry.description.is_empty() {
            "-".to_string()
        } else {
            escape(&entry.description)
        };
        out.push_str(&format!(
            "| [{}]({}) | {} | {} |\n",
            escape(&entry.title),
            entry.file,
            status,
            description
        ));
    }
    out
}

fn escape(value: &str) -> String {
    value
        .replace('|', "\\|")
        .replace('\n', " ")
        .replace('\r', " ")
}

fn replace_block(text: &str, body: &str) -> Result<String, String> {
    let normalized = text.replace("\r\n", "\n");
    let begin = normalized.find(BEGIN);
    let end = normalized.find(END);

    match (begin, end) {
        (Some(begin), Some(end)) if end > begin => {
            let head_end = begin + BEGIN.len();
            let mut out = String::with_capacity(normalized.len() + body.len());
            out.push_str(&normalized[..head_end]);
            out.push('\n');
            out.push_str(body.trim_end());
            out.push('\n');
            out.push_str(&normalized[end..]);
            Ok(out)
        }
        (None, None) => {
            let mut out = normalized.trim_end().to_string();
            if !out.is_empty() {
                out.push_str("\n\n");
            }
            out.push_str("## 文書一覧\n\n");
            out.push_str(BEGIN);
            out.push('\n');
            out.push_str(body.trim_end());
            out.push('\n');
            out.push_str(END);
            out.push('\n');
            Ok(out)
        }
        _ => Err("okf-index markers are incomplete or out of order".to_string()),
    }
}
