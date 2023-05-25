use crate::model::Entity;
use crate::storage::Storage;
use async_trait::async_trait;

pub struct SqliteStorage {}

#[async_trait]
impl Storage for SqliteStorage {
    async fn store(&self, _entities: Vec<Entity>) -> anyhow::Result<()> {
        todo!()
    }

    async fn random(&self) -> anyhow::Result<Option<Entity>> {
        todo!()
    }

    async fn search(&self, _keyword: &str, _group: Option<&str>) -> anyhow::Result<Vec<Entity>> {
        todo!()
    }

    async fn flush(&self) -> anyhow::Result<()> {
        todo!()
    }
}
