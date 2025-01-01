use anyhow::{anyhow, Result};
use regex::Regex;
use std::path::PathBuf;
use std::sync::LazyLock;

static STRICT_PATH_SEGMENT_CHECK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[-._a-z0-9]+$").unwrap());

fn is_valid_strict_path_segment(path: &str) -> bool {
    STRICT_PATH_SEGMENT_CHECK.is_match(path)
}

pub fn resolve_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::new();
    parts.into_iter().for_each(|part| {
        path.push(part);
    });
    path
}

pub fn validate_path(parts: &[&str]) -> Result<()> {
    if parts.is_empty() {
        return Err(anyhow!("Path must have at least one element"));
    } else {
        for part in parts {
            if *part == ".." || *part == "." || !is_valid_strict_path_segment(*part) {
                return Err(anyhow!("Illegal segment {} in path {:?}", part, parts));
            }
        }
    }
    Ok(())
}
