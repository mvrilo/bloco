use crate::{indexer::FileRefIndexer, FileRef, Hash, Result};
use async_trait::async_trait;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool},
    FromRow,
};
use std::str::FromStr;
use std::sync::Arc;

const STMT_CREATE_TABLES: &'static str = "
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS hashes (id primary key, hash text, hash_id integer);
CREATE TABLE IF NOT EXISTS filerefs (id primary key, name text, size integer default 0, hash text not null, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP );
COMMIT;
";

const STMT_SELECT_HASH_ID: &'static str =
    "SELECT id FROM hashes WHERE hash = ? AND hash_id IS NULL LIMIT 1";

const STMT_SELECT_FILEREFS: &'static str = "SELECT * FROM filerefs WHERE name = ?";

const STMT_SELECT_FILEREFS_HASH: &'static str = "SELECT * FROM filerefs WHERE hash = ?";

const STMT_INSERT_FILEREF: &'static str =
    "INSERT INTO filerefs (name, size, hash) VALUES (?, ?, ?)";

#[derive(sqlx::Decode, sqlx::Encode, Debug, Clone, Copy, FromRow)]
pub struct Id(i64);

#[derive(Clone)]
pub struct SqliteIndexer {
    pub db: Arc<SqlitePool>,
}

impl SqliteIndexer {
    pub fn new(pool: SqlitePool) -> Self {
        Self { db: Arc::new(pool) }
    }

    pub async fn from_dir(dir: String, dbname: String) -> Result<Self> {
        std::fs::create_dir_all(&dir).unwrap();
        let path = format!("{}/{}", dir, dbname);

        {
            #[allow(unused_must_use)]
            std::fs::File::open(&path).or(std::fs::File::create(&path));
        }

        let addr = format!("sqlite:{}", path);
        let opts = SqliteConnectOptions::from_str(&addr)?.journal_mode(SqliteJournalMode::Wal);
        let db = Self::new(SqlitePool::connect_with(opts).await?);
        db.migrate().await?;
        Ok(db)
    }

    pub async fn from_memory() -> Result<Self> {
        let opts = SqliteConnectOptions::from_str("sqlite::memory:")?;
        let db = Self::new(SqlitePool::connect_with(opts).await?);
        db.migrate().await?;
        Ok(db)
    }

    pub fn conn(&self) -> &SqlitePool {
        &self.db
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::query::<sqlx::Sqlite>(STMT_CREATE_TABLES)
            .execute(self.conn())
            .await?;
        Ok(())
    }

    pub async fn get_hash_id(&self, hash: Hash) -> Result<Id> {
        Ok(sqlx::query_as::<_, Id>(STMT_SELECT_HASH_ID)
            .bind(hash.as_hex())
            .fetch_one(self.conn())
            .await?)
    }
}

#[async_trait]
impl FileRefIndexer for SqliteIndexer {
    async fn put(&self, fr: &FileRef) -> Result<()> {
        sqlx::query(STMT_INSERT_FILEREF)
            .bind(fr.name.clone())
            .bind(fr.size)
            .bind(fr.hash.clone())
            .execute(self.conn())
            .await?;
        Ok(())
    }

    async fn get_by_name(&self, name: String) -> Result<Vec<FileRef>> {
        Ok(sqlx::query_as(STMT_SELECT_FILEREFS)
            .bind(name)
            .fetch_all(self.conn())
            .await?)
    }

    async fn get_by_hash(&self, hash: Hash) -> Result<Vec<FileRef>> {
        Ok(sqlx::query_as(STMT_SELECT_FILEREFS_HASH)
            .bind(hash.as_hex())
            .fetch_all(self.conn())
            .await?)
    }
}
