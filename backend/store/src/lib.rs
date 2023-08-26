use sqlx::{Pool, Postgres};

pub mod file;
pub mod in_memory;
pub mod postgres;
pub mod redis;

pub enum StoreType {
    InMemory,
    File,
    Postgresql,
}

pub fn new_in_memory_store() -> in_memory::InMemoryStore {
    in_memory::InMemoryStore::new()
}

pub fn new_file_store(filepath: String) -> file::FileStore {
    file::FileStore::new(filepath)
}

pub fn new_postgres_store(db: Pool<Postgres>) -> postgres::PostgresStore {
    postgres::PostgresStore::new(db)
}
