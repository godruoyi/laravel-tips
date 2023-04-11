use serde::{Deserialize, Serialize};
use crate::github;
use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub title: String,
    pub content: String,
}

pub fn parse() -> anyhow::Result<Vec<Entity>> {
    github::get_trees()?.into_iter().filter(|x| !x.is_readme()).fold(Ok(Vec::new()), |acc, x| {
        let mut acc = acc?;
        let content = x.get_content()?;
        let tips = utils::parse_tips(content)?;

        acc.extend(tips.into_iter().fold(Vec::new(), |mut e: Vec<Entity>, tip| {
            e.push(convert_tip_to_entity(tip));
            e
        }));

        Ok(acc)
    })
}

fn convert_tip_to_entity(tip: utils::Tip) -> Entity {
    //@todo adding more fields when converting from utils::Tip to Entity, such as code(php/blade/html), author, link, etc.

    Entity {
        title: tip.title,
        content: tip.content,
    }
}
