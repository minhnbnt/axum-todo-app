use anyhow::{bail, Result};

use rayon::prelude::*;

use serde::Serialize;

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::sync::RwLock;

use super::list::TodoList;

#[derive(Debug, Serialize)]
struct Task {
	id: usize,
	completed: bool,
	content: Box<str>,
}

pub struct RAMTodoList {
	size: AtomicUsize,
	list: RwLock<HashMap<usize, Task>>,
}

impl TodoList for RAMTodoList {
	fn new_task(&self, mut content: String) -> Result<()> {
		let mut list = self.list.try_write()?;

		content.shrink_to_fit();

		let task = Task {
			completed: false,
			content: content.into(),
			id: self.size.fetch_add(1, Ordering::SeqCst),
		};

		list.insert(task.id, task);
		Ok(())
	}

	fn remove_task(&self, id: usize) -> Result<()> {
		let mut list = self.list.try_write()?;

		if list.remove(&id).is_none() {
			bail!("Failed to remove task {0}: Task {0} not found.", id);
		}

		Ok(())
	}

	fn change_task(&self, id: usize, mut content: String) -> Result<()> {
		let mut list = self.list.try_write()?;

		content.shrink_to_fit();
		if let Some(task) = list.get_mut(&id) {
			task.content = content.into();
			task.completed = false;
		} else {
			bail!("Failed to rename task {0}: Task {0} not found.", id);
		}

		Ok(())
	}

	fn mark_completed(&self, id: usize) -> Result<()> {
		let mut list = self.list.try_write().unwrap();

		if let Some(task) = list.get_mut(&id) {
			task.completed = true;
		} else {
			bail!("Failed to complete task {0}: Task {0} not found.", id);
		}

		Ok(())
	}

	fn get_tasks(&self) -> Result<String> {
		let list = self.list.try_read().unwrap();
		let mut tasks: Vec<&Task> = list.par_iter().map(|(_, task)| task).collect();

		tasks.sort_by(|a, b| a.id.cmp(&b.id));

		Ok(serde_json::to_string(&tasks)?)
	}
}

impl RAMTodoList {
	pub fn new() -> Self {
		RAMTodoList {
			size: AtomicUsize::new(0),
			list: RwLock::new(HashMap::new()),
		}
	}
}
