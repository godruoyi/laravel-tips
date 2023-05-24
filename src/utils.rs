use crate::model::Tip;
use base64::{engine::general_purpose, Engine};

pub fn base64_decode(c: String) -> anyhow::Result<String> {
    // github api response always has a newline between each base64 part
    let c = c.replace('\n', "");
    let c = c.replace("\\n", "");

    let decoded = general_purpose::STANDARD.decode(c.as_bytes())?;

    String::from_utf8(decoded).map_err(|e| e.into())
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
}
