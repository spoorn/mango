use anyhow::{anyhow, Result};
use regex::Regex;
use std::path::PathBuf;
use std::sync::LazyLock;

static STRICT_PATH_SEGMENT_CHECK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[-._a-z0-9]+$").unwrap());

/// We have a custom function to decompose a path so we can run our own validation checks against it
/// and handle "virtual" paths that point to inlined resources in the binary that may not exist on
/// the filesystem.
pub fn decompose_path(path: &String) -> Result<Vec<String>> {
    match path.find("/") {
        None => match path.as_str() {
            "" | "." | ".." => Err(anyhow!("Invalid path '{}'", path)),
            _ => {
                if !is_valid_strict_path_segment(&path) {
                    Err(anyhow!("Invalid path '{}'", path))
                } else {
                    Ok(vec![path.clone()])
                }
            }
        },
        Some(mut sep) => {
            let mut res = Vec::new();
            let mut start = 0;

            loop {
                let left = &path[start..sep];
                if left == "" || left == "." || left == ".." || !is_valid_strict_path_segment(left)
                {
                    return Err(anyhow!("Invalid segment '{}' in path '{}'", left, path));
                }

                res.push(left.to_string());

                if sep == path.len() {
                    break;
                }

                start = sep + 1;
                sep = match path[start..].find("/") {
                    None => path.len(),
                    Some(index) => index,
                };
            }

            Ok(res)
        }
    }
}

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
