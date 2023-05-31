use crate::model::Entity;
use crate::storage::Storage;
use crate::utils::normalize_path;
use async_trait::async_trait;
use rusqlite::{params, Connection};
use std::path::PathBuf;

const SQL_CREATE_TABLE: &str = r#"
    ;
    CREATE TABLE IF NOT EXISTS laravel_tips (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        content TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );
"#;

const SQL_INSERT: &str = r#"
    INSERT INTO laravel_tips (title, content) VALUES (?, ?);
"#;

#[cfg_attr(test, derive(Debug))]
pub struct SqliteStorage {
    path: Option<PathBuf>,
}

impl SqliteStorage {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path }
    }

    fn path(&self) -> anyhow::Result<String> {
        normalize_path(".db3".to_string(), self.path.clone())
    }

    /// Open a connection to the database.
    ///
    /// @TODO if the connection is already open, return it.
    fn connection(&self) -> anyhow::Result<Connection> {
        let path = self.path()?;

        let con = Connection::open(path)?;

        Ok(con)
    }

    fn create_table_if_not_exists(&self) -> anyhow::Result<()> {
        let con = self.connection()?;
        con.execute(SQL_CREATE_TABLE, [])?;

        Ok(())
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn store(&self, entities: Vec<Entity>) -> anyhow::Result<()> {
        self.flush().await?;

        let con = self.connection()?;
        let mut stmt = con.prepare_cached(SQL_INSERT)?;

        for entity in entities {
            stmt.execute(params![entity.title, entity.content])?;
        }

        Ok(())
    }

    async fn random(&self) -> anyhow::Result<Option<Entity>> {
        self.create_table_if_not_exists()?;

        let con = self.connection()?;
        let mut query =
            con.prepare("SELECT id, title, content FROM laravel_tips ORDER BY RANDOM() LIMIT 1")?;

        let entity = query.query_row([], |row| {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let content: String = row.get(2)?;
            Ok(Entity {
                id: id.to_string(),
                title,
                content,
            })
        })?;

        Ok(Some(entity))
    }

    async fn search(&self, keyword: &str, _group: Option<&str>) -> anyhow::Result<Vec<Entity>> {
        self.create_table_if_not_exists()?;

        let con = self.connection()?;
        let mut query = con.prepare_cached(
            "SELECT id, title, content FROM laravel_tips WHERE title LIKE ? OR content LIKE ?",
        )?;

        let keyword = format!("%{}%", keyword);
        let rows = query.query_map(params![keyword.clone(), keyword], |row| {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let content: String = row.get(2)?;
            Ok(Entity {
                id: id.to_string(),
                title,
                content,
            })
        })?;

        let mut entities = Vec::new();
        for row in rows {
            entities.push(row?);
        }

        Ok(entities)
    }

    async fn flush(&self) -> anyhow::Result<()> {
        self.create_table_if_not_exists()?;

        let con = self.connection()?;
        con.execute("DELETE FROM laravel_tips", [])?;

        Ok(())
    }
}
