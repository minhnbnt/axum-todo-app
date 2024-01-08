use rayon::prelude::*;

use serde::Serialize;

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use tokio::sync::RwLock;

use tracing::error;

#[derive(Debug, Serialize)]
struct Task {
	id: usize,
	completed: bool,
	content: Box<str>,
}

pub struct TodoList {
	size: AtomicUsize,
	list: RwLock<HashMap<usize, Task>>,
}

impl TodoList {
	pub fn new_task(&self, mut content: String) {
		let mut list = self.list.try_write().unwrap();

		content.shrink_to_fit();

		let task = Task {
			completed: false,
			content: content.into(),
			id: self.size.fetch_add(1, Ordering::SeqCst),
		};

		list.insert(task.id, task);
	}

	pub fn remove_task(&self, id: usize) {
		let mut list = self.list.try_write().unwrap();

		if list.remove(&id).is_none() {
			error!("Failed to remove task with id: {}", id);
		}
	}

	pub fn change_task(&self, id: usize, mut content: String) {
		let mut list = self.list.try_write().unwrap();

		content.shrink_to_fit();
		if let Some(task) = list.get_mut(&id) {
			task.content = content.into();
			task.completed = false;
		}
	}

	pub fn mark_completed(&self, id: usize) {
		let mut list = self.list.try_write().unwrap();

		if let Some(task) = list.get_mut(&id) {
			task.completed = true;
		}
	}

	pub fn get_tasks(&self) -> Result<String, serde_json::Error> {
		let list = self.list.try_read().unwrap();
		let mut tasks: Vec<&Task> = list.par_iter().map(|(_, task)| task).collect();

		tasks.sort_by(|a, b| a.id.cmp(&b.id));

		serde_json::to_string(&tasks)
	}

	pub fn new() -> Self {
		TodoList {
			size: AtomicUsize::new(0),
			list: RwLock::new(HashMap::new()),
		}
	}
}
