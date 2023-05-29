use crate::model::Entity;
use crate::storage::Storage;
use crate::utils::normalize_path;
use anyhow::anyhow;
use async_trait::async_trait;
use rand::prelude::SliceRandom;
use std::path::PathBuf;

pub struct FileStorage {
    path: Option<PathBuf>,
    suffix: Option<String>,
}

impl FileStorage {
    pub fn new(path: Option<PathBuf>, suffix: Option<String>) -> Self {
        Self { path, suffix }
    }

    fn path(&self) -> anyhow::Result<String> {
        let suffix = self
            .suffix
            .clone()
            .unwrap_or_else(|| "tips.json".to_string());

        normalize_path(suffix, self.path.clone())
    }
}

#[async_trait]
impl Storage for FileStorage {
    async fn store(&self, entities: Vec<Entity>) -> anyhow::Result<()> {
        let json = serde_json::to_string(&entities)?;

        std::fs::write(self.path()?, json)?;

        Ok(())
    }

    async fn random(&self) -> anyhow::Result<Option<Entity>> {
        let path = self.path()?;
        let m = std::fs::metadata(&path);

        if m.is_err() || !m.unwrap().is_file() {
            return Err(anyhow!("can't load tips from {}, try [sync] first", &path));
        }

        let json = std::fs::read_to_string(&path)?;
        let entities = serde_json::from_str::<Vec<Entity>>(&json)?;

        if entities.is_empty() {
            return Ok(None);
        }

        let mut rng = rand::thread_rng();
        let entity = entities.choose(&mut rng).unwrap();

        Ok(Some(Entity {
            id: entity.id.clone(),
            title: entity.title.clone(),
            content: entity.content.clone(),
        }))
    }

    async fn search(&self, _: &str, _: Option<&str>) -> anyhow::Result<Vec<Entity>> {
        Err(anyhow!(
            "file storage does not support search, please use sqlite storage"
        ))
    }

    async fn flush(&self) -> anyhow::Result<()> {
        let path = self.path()?;
        let m = std::fs::metadata(&path);

        // cannot flush if file not exists or something wrong
        if m.is_err() || !m.unwrap().is_file() {
            return Ok(());
        }

        if let Err(err) = std::fs::remove_file(&path) {
            return Err(anyhow!("remove file failed: {}", err));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search() {
        let storage = FileStorage::new(None, None);
        let result = storage.search("test", None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_store() {
        let storage = FileStorage::new(Some(file_path()), None);
        let result = storage.store(vec![]).await;

        storage.flush().await.expect("flush failed");

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_random() {
        let storage = FileStorage::new(Some(file_path()), None);
        storage.flush().await.expect("flush failed before random");

        let result = storage.random().await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("can't load tips from"));

        storage.store(vec![]).await.expect("store failed");
        let result = storage.random().await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        let v = vec![Entity {
            id: "1".to_string(),
            title: "test".to_string(),
            content: "test".to_string(),
        }];

        storage.store(v).await.expect("store failed");
        let entity = storage.random().await.expect("random failed");

        assert!(entity.is_some());
        assert_eq!(entity.unwrap().id, "1");

        storage.flush().await.expect("flush failed");
    }

    fn file_path() -> PathBuf {
        std::env::current_dir().unwrap().join("testdata")
    }

    #[test]
    fn test_none_unwrap() {
        let a = None;
        let a = a.unwrap_or(1);

        assert_eq!(1, a);
    }
}
