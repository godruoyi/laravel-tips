use serde::{Deserialize};
use crate::http::http_get;

const GITHUB_TREES_API: &str = "https://api.github.com/repos/LaravelDaily/laravel-tips/git/trees/master?recursive=1";

pub fn get_trees() -> anyhow::Result<Vec<Tree>> {
    let res = http_get::<Trees>(GITHUB_TREES_API)?;

    Ok(res.tree)
}

#[derive(Debug, Deserialize)]
struct Trees {
    tree: Vec<Tree>,
}

#[derive(Debug, Deserialize)]
pub struct Tree {
    path: String,
    url: String,
}

impl Tree {
    fn new(path: String, url: String) -> Self {
        Self { path, url }
    }

    pub fn is_readme(&self) -> bool {
        self.path.to_uppercase() == "README.md"
    }

    pub fn get_content(&self) -> anyhow::Result<String> {
        #[derive(Deserialize)]
        struct Content {
            content: String,
        }

        let res = http_get::<Content>(&self.url)?;

        Ok(res.content)
    }
}
