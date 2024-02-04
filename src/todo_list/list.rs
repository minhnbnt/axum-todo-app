use crate::error::AppResult;

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
	async fn new_task(&self, content: String) -> AppResult<()>;

	async fn get_tasks(&self) -> AppResult<Json<Vec<Task>>>;

	async fn change_task(&self, id: u32, content: String) -> AppResult<()>;

	async fn mark_completed(&self, id: u32) -> AppResult<()>;

	async fn remove_task(&self, id: u32) -> AppResult<()>;
}
