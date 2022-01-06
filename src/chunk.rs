#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Chunk {
    pub hash: String,
}
