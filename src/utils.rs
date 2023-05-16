use base64::{engine::general_purpose, Engine};
use home::home_dir;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub fn base64_decode(c: String) -> anyhow::Result<String> {
    // github api response always has a newline between each base64 part
    let c = c.replace('\n', "");
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
    Ok(base64_decode(c)?
        .lines()
        .fold((None, Vec::new()), process_line)
        .1)
}

fn process_line(mut state: (Option<Tip>, Vec<Tip>), line: &str) -> (Option<Tip>, Vec<Tip>) {
    match state.0.take() {
        Some(mut entity) => {
            if line.starts_with("###") {
                state.1.push(entity);
                let title = line.trim_start_matches("###").trim().to_string();
                state.0 = Some(Tip {
                    title,
                    content: String::new(),
                });
            } else {
                entity.content.push_str(line);
                entity.content.push('\n');
                state.0 = Some(entity);
            }
        }
        None => {
            if line.starts_with("###") {
                let title = line.trim_start_matches("###").trim().to_string();
                state.0 = Some(Tip {
                    title,
                    content: String::new(),
                });
            }
        }
    }
    state
}

pub fn save_tips_to_disk<T: Serialize>(path: Option<String>, tips: &Vec<T>) -> anyhow::Result<()> {
    let json = serde_json::to_string(&tips)?;
    std::fs::write(normalize_path(path, "tips.json"), json)?;

    Ok(())
}

pub fn load_tips_from_disk<T: DeserializeOwned>(path: Option<String>) -> anyhow::Result<Vec<T>> {
    let path = normalize_path(path, "tips.json");
    let json = std::fs::read_to_string(path)?;

    Ok(serde_json::from_str(&json)?)
}

fn normalize_path(path: Option<String>, suffix: &str) -> String {
    PathBuf::from(path.unwrap_or_else(|| {
        let p = format!("{}/.laravel", home_dir().unwrap().to_str().unwrap());
        if let Err(err) = std::fs::metadata(&p) {
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    std::fs::create_dir(&p).unwrap();
                }
                _ => panic!("create dir failed: {}", err),
            }
        }
        p
    }))
    .join(suffix)
    .to_str()
    .unwrap()
    .to_string()
}

#[cfg(test)]
mod test_base {
    use super::*;

    #[test]
    fn test_base64_decode_from_local_file() {
        let encode_content = std::fs::read_to_string("testdata/api_base64.md");
        assert!(encode_content.is_ok());

        let x = base64_decode(encode_content.unwrap());
        assert!(x.is_ok());
    }

    #[test]
    fn test_parse_tips() {
        let encode_content = std::fs::read_to_string("testdata/api_base64.md");
        assert!(encode_content.is_ok());

        let x = parse_tips(encode_content.unwrap());
        assert!(x.is_ok());
    }

    #[test]
    fn test_can_normalize_path() {
        let path = Some("/tmp".to_string());
        let suffix = "tips.json";
        let p = normalize_path(path, suffix);

        assert_eq!(p, "/tmp/tips.json");
    }

    #[test]
    fn test_can_save_and_load_tips_from_disk() {
        let tips = vec![Tip {
            title: "test".to_string(),
            content: "test".to_string(),
        }];

        assert!(save_tips_to_disk(None, &tips).is_ok());
        let p = normalize_path(None, "tips.json");
        assert!(std::fs::metadata(&p).is_ok());

        let tips = load_tips_from_disk::<Tip>(None);
        assert!(tips.is_ok());
        assert_eq!(tips.unwrap()[0].title, "test");

        std::fs::remove_file(p).unwrap();
    }
}
