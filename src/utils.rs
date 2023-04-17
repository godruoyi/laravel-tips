use base64::{Engine, engine::{general_purpose}};
use serde::de::{DeserializeOwned};
use serde::{Deserialize, Serialize};

pub fn base64_decode(c: String) -> anyhow::Result<String> {
    // github api response always has a newline between each base64 part
    // @todo find a better way to refactor this
    let c = c.replace("\n", "");
    let c = c.replace("\\n", "");

    let decoded = general_purpose::STANDARD.decode(c.as_bytes())?;

    String::from_utf8(decoded).map_err(|e| e.into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tip {
    pub title: String,
    pub content: String,
}

pub fn parse_tips(c: String) -> anyhow::Result<Vec<Tip>> {
    Ok(base64_decode(c)?.lines().fold((None, Vec::new()), process_line).1)
}

fn process_line(mut state: (Option<Tip>, Vec<Tip>), line: &str) -> (Option<Tip>, Vec<Tip>) {
    match state.0.take() {
        Some(mut entity) => {
            if line.starts_with("###") {
                state.1.push(entity);
                let title = line.trim_start_matches("###").trim().to_string();
                state.0 = Some(Tip { title, content: String::new() });
            } else {
                entity.content.push_str(line);
                entity.content.push('\n');
                state.0 = Some(entity);
            }
        }
        None => {
            if line.starts_with("###") {
                let title = line.trim_start_matches("###").trim().to_string();
                state.0 = Some(Tip { title, content: String::new() });
            }
        }
    }
    state
}

pub fn save_tips_to_disk<T: Serialize>(tips: &Vec<T>) -> anyhow::Result<()> {
    let json = serde_json::to_string(&tips)?;
    std::fs::write("tips.json", json)?;
    Ok(())
}

pub fn load_tips_from_disk<T: DeserializeOwned>() -> anyhow::Result<Vec<T>> {
    let json = std::fs::read_to_string("tips.json")?;

    Ok(serde_json::from_str(&json)?)
}

#[cfg(test)]
mod test_base {
    use crate::http;
    use serde_json::Value;
    use std::collections;
    use super::*;

    #[test]
    fn test_base64_decode_from_local_file() {
        let encode_content = std::fs::read_to_string("testdata/api_base64.md");
        assert!(encode_content.is_ok());

        let x = base64_decode(encode_content.unwrap());
        assert!(x.is_ok());
    }

    #[test]
    fn test_base64_decode_from_github_api() {
        let resp = http::http_get::<collections::HashMap<String, Value>>("https://api.github.com/repos/LaravelDaily/laravel-tips/git/blobs/5b7d0d2cc4f6865b8492e47ed6eb3d0beecd4482");
        assert!(resp.is_ok());

        let encode_content = resp.unwrap().get("content").unwrap().as_str().unwrap().to_string();

        let x = base64_decode(encode_content);
        assert!(x.is_ok());
    }

    #[test]
    fn test_parse_tips() {
        let encode_content = std::fs::read_to_string("testdata/api_base64.md");
        assert!(encode_content.is_ok());

        let x = parse_tips(encode_content.unwrap());
        assert!(x.is_ok());
    }
}