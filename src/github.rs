use crate::utils;
use crate::utils::{save_tips_to_disk, Tip};
use indicatif::ProgressBar;
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

const ENV_LARAVEL_TIPS_ACCESS_TOKEN: &str = "LARAVEL_TIPS_ACCESS_TOKEN";
const ENV_HTTP_USER_AGENT: &str = "LARAVEL_TIPS_HTTP_USER_AGENT";
const ENV_HTTP_ACCEPR: &str = "LARAVEL_TIPS_HTTP_ACCEPR";

const GITHUB_TREES_API: &str =
    "https://api.github.com/repos/LaravelDaily/laravel-tips/git/trees/master?recursive=1";

#[derive(Debug, Deserialize)]
struct Trees {
    tree: Vec<Tree>,
}

/// The tree struct for github api response
///
/// see [git/trees](https://docs.github.com/en/rest/git/trees?apiVersion=2022-11-28#get-a-tree)
#[derive(Debug, Deserialize)]
pub struct Tree {
    path: String,
    url: String,
    size: usize,
}

impl Tree {
    /// Check if the file is readme.md
    pub fn is_readme(&self) -> bool {
        self.path.to_uppercase() == "README.md"
    }

    /// Get the file size, bytes
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Get the file content, note that the content is base64 encoded
    pub fn get_content(&self) -> anyhow::Result<String> {
        #[derive(Deserialize)]
        struct Content {
            content: String,
        }

        let res = http_get::<Content>(&self.url)?;

        Ok(res.content)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub title: String,
    pub content: String,
}

/// Get all tips file from the github repository
///
/// We will get the all files from the LaravelDaily/laravel-tips repository
pub fn get_laravel_tips_trees() -> anyhow::Result<Vec<Tree>> {
    let res = http_get::<Trees>(GITHUB_TREES_API)?;

    Ok(res.tree)
}

pub fn get_get_laravel_tips_trees_with_size() -> anyhow::Result<(Vec<Tree>, u64)> {
    let trees = get_laravel_tips_trees()?;
    let total: usize = trees
        .iter()
        .filter(|t| !t.is_readme())
        .map(|t| t.get_size())
        .sum();

    Ok((trees, total as u64))
}

pub fn process_trees(path: Option<String>, trees: Vec<Tree>) -> impl Fn(&mut ProgressBar) {
    move |pb: &mut ProgressBar| {
        let mut entities: Vec<Entity> = Vec::new();

        for tree in trees.iter() {
            if tree.is_readme() {
                continue;
            }

            if let Ok(content) = tree.get_content() {
                if let Ok(tips) = utils::parse_tips(content) {
                    entities.extend(convert_tips_to_entities(tips));
                }
            }

            pb.set_position(pb.position() + tree.get_size() as u64);
        }

        save_tips_to_disk(path.clone(), &entities).unwrap();
    }
}

fn convert_tips_to_entities(tips: Vec<Tip>) -> Vec<Entity> {
    tips.into_iter()
        //@todo adding more fields when converting from utils::Tip to Entity, such as code(php/blade/html), author, link, etc.
        .map(|t| Entity {
            title: t.title,
            content: t.content,
        })
        .collect()
}

/// Basic http get method,
fn http_get<T: DeserializeOwned>(url: &str) -> anyhow::Result<T> {
    let mut headers = reqwest::header::HeaderMap::new();
    let agent = std::env::var(ENV_HTTP_USER_AGENT).unwrap_or_else(|_| "laravel-tips".to_string());
    let accept = std::env::var(ENV_HTTP_ACCEPR)
        .unwrap_or_else(|_| "application/vnd.github.v3+json".to_string());

    headers.insert(
        reqwest::header::USER_AGENT,
        HeaderValue::from_str(agent.as_str()).unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        HeaderValue::from_str(accept.as_str()).unwrap(),
    );

    if let Ok(token) = std::env::var(ENV_LARAVEL_TIPS_ACCESS_TOKEN) {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap(),
        );
    }

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client.get(url).send()?.json::<T>()?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::base64_decode;
    use serde_json::Value;
    use std::collections;

    #[test]
    fn test_base64_decode_from_github_api() {
        let resp = http_get::<collections::HashMap<String, Value>>("https://api.github.com/repos/LaravelDaily/laravel-tips/git/blobs/5b7d0d2cc4f6865b8492e47ed6eb3d0beecd4482");
        assert!(resp.is_ok());

        let encode_content = resp
            .unwrap()
            .get("content")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let x = base64_decode(encode_content);
        assert!(x.is_ok());
    }
}
