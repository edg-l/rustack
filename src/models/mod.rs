//! Models
//!
//! The database models used in the app.
//!
//! Check out https://github.com/launchbadge/sqlx

#![allow(dead_code)]

pub use sqlx::types::Uuid;
use sqlx::{PgPool, Result, FromRow};

#[derive(FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub content: String,
}

impl Todo {
    pub fn new(title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            content,
        }
    }

    // TODO: insert, update, delete, get...
    
    pub async fn insert(&self, pool: &PgPool) -> Result<()> {
        sqlx::query("INSERT INTO todo (id, title, content) VALUES (?, ?, ?)")
            .bind(&self.id)
            .bind(&self.title)
            .bind(&self.content)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get(pool: &PgPool, uuid: &Uuid) -> Result<Todo> {
        sqlx::query_as("SELECT * FROM todo WHERE id = ?")
            .bind(uuid)
            .fetch_one(pool)
            .await
    }
}
