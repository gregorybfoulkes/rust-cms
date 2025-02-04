// This file manages database connections and interactions, including establishing a connection pool and executing queries.

use sqlx::{Pool, Postgres};
use std::env;

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")?;
        let pool = Pool::<Postgres>::connect(&database_url).await?;
        Ok(Database { pool })
    }

    pub async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error> {
        sqlx::query(query).execute(&self.pool).await?;
        Ok(())
    }

    // Additional database interaction methods can be added here
}