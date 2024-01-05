use serde::Serialize;

use std::collections::HashMap;
use std::sync::RwLock;

use tracing::error;

#[derive(Debug, Serialize)]
struct Task {
	id: usize,
	completed: bool,
	content: Box<str>,
}

pub struct TodoList {
	size: usize,
	list: RwLock<HashMap<usize, Task>>,
}

#[allow(dead_code)]
impl TodoList {
	fn new_task(&mut self, mut content: String) {
		content.shrink_to_fit();

		let task = Task {
			id: self.size,
			content: content.into(),
			completed: false,
		};

		if let Ok(mut list) = self.list.write() {
			list.insert(task.id, task);
			self.size += 1;
		}
	}

	fn remove_task(&mut self, id: usize) {
		let Ok(mut list) = self.list.write() else {
			return;
		};

		if list.remove(&id).is_none() {
			error!("Failed to remove task with id: {}", id);
		}
	}

	fn change_task(&mut self, mut content: String, id: usize) {
		let Ok(mut list) = self.list.write() else {
			return;
		};

		content.shrink_to_fit();
		if let Some(task) = list.get_mut(&id) {
			task.content = content.into();
			task.completed = false;
		}
	}

	fn mark_completed(&mut self, id: usize) {
		let Ok(mut list) = self.list.write() else {
			return;
		};

		if let Some(task) = list.get_mut(&id) {
			task.completed = true;
		}
	}

	fn to_json(&self) -> Result<String, String> {
		let Ok(list) = self.list.read() else {
			return Err("Failed to read the database.".into());
		};

		let tasks: Vec<&Task> = list.values().collect();

		serde_json::to_string(&tasks).map_err(|e| format!("Failed to convert data to json: {}", e))
	}

	pub fn new() -> Self {
		TodoList {
			size: 0,
			list: RwLock::new(HashMap::new()),
		}
	}
}
