use anyhow::Result;

use axum::{async_trait, Json};

use serde::Serialize;

use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Task {
	pub id: u32,
	pub completed: bool,
	pub description: Box<str>,
}

#[async_trait]
pub trait TodoList {
	async fn new_task(&self, content: String) -> Result<()>;

	async fn get_tasks(&self) -> Result<Json<Vec<Task>>>;

	async fn change_task(&self, id: u32, content: String) -> Result<()>;
	async fn mark_completed(&self, id: u32) -> Result<()>;

	async fn remove_task(&self, id: u32) -> Result<()>;
}
