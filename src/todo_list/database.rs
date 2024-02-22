use crate::error::AppResult;

use axum::{async_trait, Json};

use sqlx::mysql::MySqlPool;

use std::env;

use super::list::{Task, TodoList};

pub struct Database {
	connection: MySqlPool,
}

impl Database {
	pub async fn new() -> AppResult<Self> {
		let url = env::var("DATABASE_URL")?;
		let pool = MySqlPool::connect(&url).await?;

		Ok(Self { connection: pool })
	}
}

#[async_trait]
impl TodoList for Database {
	async fn new_task(&self, content: &str) -> AppResult<()> {
		sqlx::query!("INSERT INTO Todo (description) VALUES (?)", content)
			.execute(&self.connection)
			.await?;

		Ok(())
	}

	async fn get_tasks(&self) -> AppResult<Json<Vec<Task>>> {
		let tasks = sqlx::query_as::<_, Task>("SELECT * FROM Todo ORDER BY id")
			.fetch_all(&self.connection)
			.await?;

		Ok(Json(tasks))
	}

	async fn change_task(&self, id: u32, content: &str) -> AppResult<()> {
		sqlx::query("UPDATE Todo SET description=$1 WHERE id=$2")
			.bind(content)
			.bind(id)
			.execute(&self.connection)
			.await?;

		Ok(())
	}
	async fn mark_completed(&self, id: u32) -> AppResult<()> {
		sqlx::query!("UPDATE Todo SET completed=true WHERE id=?", id)
			.execute(&self.connection)
			.await?;

		Ok(())
	}

	async fn remove_task(&self, id: u32) -> AppResult<()> {
		sqlx::query!("DELETE FROM Todo WHERE id=?", id)
			.execute(&self.connection)
			.await?;

		Ok(())
	}
}
