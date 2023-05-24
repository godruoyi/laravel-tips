use crate::model::Entity;
use crate::storage::file::FileStorage;
use async_trait::async_trait;
use std::path::PathBuf;

mod file;
mod sqlite;

#[async_trait]
pub trait Storage {
    async fn store(&self, entities: Vec<Entity>) -> anyhow::Result<()>;
    async fn random(&self) -> anyhow::Result<Option<Entity>>;
    async fn search(&self, keyword: &str, group: Option<&str>) -> anyhow::Result<Vec<Entity>>;
    async fn flush(&self) -> anyhow::Result<()>;
}

pub fn new_storage(path: Option<String>) -> Box<dyn Storage> {
    // @todo use parameter to decide which storage to use

    let p = path.map(PathBuf::from);
    let storage = FileStorage::new(p, None);

    Box::new(storage)
}
