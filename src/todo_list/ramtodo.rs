use anyhow::{bail, Ok, Result};
use axum::{async_trait, Json};

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use tokio::sync::RwLock;

use super::list::{Task, TodoList};

pub struct RAMTodoList {
	size: AtomicU32,
	list: RwLock<HashMap<u32, Task>>,
}

#[async_trait]
impl TodoList for RAMTodoList {
	async fn new_task(&self, content: String) -> Result<()> {
		let mut list = self.list.try_write()?;

		let task = Task {
			completed: false,
			description: content.into(),
			id: self.size.fetch_add(1, Ordering::SeqCst),
		};

		list.insert(task.id, task);
		Ok(())
	}

	async fn remove_task(&self, id: u32) -> Result<()> {
		let mut list = self.list.try_write()?;

		if list.remove(&id).is_none() {
			bail!("Failed to remove task {0}: Task {0} not found.", id);
		}

		Ok(())
	}

	async fn change_task(&self, id: u32, mut content: String) -> Result<()> {
		let mut list = self.list.try_write()?;

		content.shrink_to_fit();
		if let Some(task) = list.get_mut(&id) {
			task.description = content.into();
			task.completed = false;
		} else {
			bail!("Failed to rename task {0}: Task {0} not found.", id);
		}

		Ok(())
	}

	async fn mark_completed(&self, id: u32) -> Result<()> {
		let mut list = self.list.try_write().unwrap();

		if let Some(task) = list.get_mut(&id) {
			task.completed = true;
		} else {
			bail!("Failed to complete task {0}: Task {0} not found.", id);
		}

		Ok(())
	}

	async fn get_tasks(&self) -> Result<Json<Vec<Task>>> {
		let list = self.list.try_read()?;

		let mut tasks: Vec<_> = list.iter().map(|(_, &task)| task).collect();
		tasks.sort_by(|a, b| a.id.cmp(&b.id));

		Ok(Json(tasks))
	}
}

impl RAMTodoList {
	pub fn new() -> Self {
		RAMTodoList {
			size: AtomicU32::new(0),
			list: RwLock::new(HashMap::new()),
		}
	}
}
