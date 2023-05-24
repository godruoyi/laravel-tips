use crate::model::Entity;
use crate::storage::Storage;
use anyhow::anyhow;
use async_trait::async_trait;
use home::home_dir;
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

    fn normalize_path(&self) -> String {
        self.path
            .clone()
            .unwrap_or_else(create_default_laravel_directory)
            .join(self.suffix.clone().unwrap_or_else(|| ".tips".to_string()))
            .to_str()
            .unwrap()
            .to_string()
    }
}

#[async_trait]
impl Storage for FileStorage {
    async fn store(&self, entities: Vec<Entity>) -> anyhow::Result<()> {
        let json = serde_json::to_string(&entities)?;
        std::fs::write(self.normalize_path(), json)?;

        Ok(())
    }

    async fn random(&self) -> anyhow::Result<Option<Entity>> {
        let path = self.normalize_path();
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
        let path = self.normalize_path();
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

fn create_default_laravel_directory() -> PathBuf {
    let p = format!("{}/.laravel", home_dir().unwrap().to_str().unwrap());
    if let Err(err) = std::fs::metadata(&p) {
        match err.kind() {
            std::io::ErrorKind::NotFound => {
                std::fs::create_dir(&p).unwrap();
            }
            _ => panic!("cannot create directory, path: {}, err: {}", &p, err),
        }
    }

    PathBuf::from(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_normalize_path() {
        let storage = FileStorage::new(None, None);
        let p = storage.normalize_path();

        assert_eq!(
            p,
            format!("{}/.laravel/.tips", home_dir().unwrap().to_str().unwrap())
        );
    }

    #[test]
    fn test_can_normalize_path_with_path() {
        let storage = FileStorage::new(Some(PathBuf::from("path1/path2")), None);
        let p = storage.normalize_path();

        assert_eq!(p, "path1/path2/.tips");
    }

    #[test]
    fn test_can_normalize_path_with_path_and_suffix() {
        let storage = FileStorage::new(
            Some(PathBuf::from("path1/path2")),
            Some(".laravel".to_string()),
        );
        let p = storage.normalize_path();

        assert_eq!(p, "path1/path2/.laravel");
    }

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
