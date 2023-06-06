use crate::model::{Entity, Tip};
use crate::{log, utils};
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;
use serde::Deserialize;

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
}

impl Tree {
    /// Check if the file is readme.md
    pub fn is_readme(&self) -> bool {
        self.path == "README.md"
    }

    /// Get the file content, note that the content is base64 encoded
    pub async fn get_content(&self, quiet: bool) -> anyhow::Result<String> {
        if !quiet {
            log!(format!(" parsing file: {}", &self.path));
        }

        #[derive(Deserialize)]
        struct Content {
            content: String,
        }

        let res = http_get::<Content>(&self.url).await?;

        Ok(res.content)
    }
}

pub async fn parse_all_laravel_tips(quiet: bool) -> anyhow::Result<Vec<Entity>> {
    // 1. get all tips file from the laravel-tips repository
    let trees: Vec<Tree> = get_laravel_tips_trees().await?;
    let mut entities: Vec<Entity> = Vec::new();

    // 2. generate the tasks for each file
    let tasks: Vec<_> = trees
        .iter()
        .filter(|tree| !tree.is_readme())
        .map(|t| t.get_content(quiet))
        .collect();

    // 3. wait for all tasks to complete
    let result = futures::future::join_all(tasks).await;
    for content in result.into_iter().flatten() {
        if let Ok(tips) = utils::parse_tips(content) {
            entities.extend(convert_tips_to_entities(tips));
        }
    }

    Ok(entities)
}

/// Get all tips file from the github repository
///
/// We will get the all files from the LaravelDaily/laravel-tips repository
async fn get_laravel_tips_trees() -> anyhow::Result<Vec<Tree>> {
    let res = http_get::<Trees>(GITHUB_TREES_API).await?;

    Ok(res.tree)
}

fn convert_tips_to_entities(tips: Vec<Tip>) -> Vec<Entity> {
    tips.into_iter()
        //@todo adding more fields when converting from utils::Tip to Entity, such as code(php/blade/html), author, link, etc.
        .map(|t| Entity {
            id: "".to_string(),
            title: t.title,
            content: t.content,
        })
        .collect()
}

/// Basic http get method,
async fn http_get<T: DeserializeOwned>(url: &str) -> anyhow::Result<T> {
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

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client.get(url).send().await?.json::<T>().await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::base64_decode;
    use serde_json::Value;
    use std::collections;

    #[tokio::test]
    async fn test_base64_decode_from_github_api() {
        let resp = http_get::<collections::HashMap<String, Value>>("https://api.github.com/repos/LaravelDaily/laravel-tips/git/blobs/5b7d0d2cc4f6865b8492e47ed6eb3d0beecd4482").await;

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
