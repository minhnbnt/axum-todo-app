use crate::todo_list::TodoList;

use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};

use serde::Deserialize;

use tracing::debug;

pub struct TodoListRouter {
	todo_list: TodoList,
}

// todo: remove static lifetime
impl TodoListRouter {
	pub fn new() -> Self {
		Self {
			todo_list: TodoList::new(),
		}
	}

	fn new_task(&'static self) -> Router {
		let function = move |Json(payload): Json<NewTaskPayload>| async move {
			debug!("New task: {}", payload.content);
			self.todo_list.new_task(payload.content);
			(StatusCode::CREATED, "Task created")
		};

		Router::new().route("/", post(function))
	}

	fn remove_task(&'static self) -> Router {
		let function = move |Json(payload): Json<TaskIDPayload>| async move {
			debug!("Remove task: {}", payload.id);
			self.todo_list.remove_task(payload.id);
		};
		Router::new().route("/", delete(function))
	}

	fn change_task(&'static self) -> Router {
		let function = move |Json(payload): Json<ChangeTaskPayload>| async move {
			debug!("Change task: {}", payload.id);
			self.todo_list.change_task(payload.id, payload.content);
		};
		Router::new().route("/", patch(function))
	}

	fn mark_completed(&'static self) -> Router {
		let function = move |Json(payload): Json<TaskIDPayload>| async move {
			debug!("Mark completed: {}", payload.id);
			self.todo_list.mark_completed(payload.id);
		};
		Router::new().route("/", patch(function))
	}

	fn get_tasks(&'static self) -> Router {
		Router::new().route(
			"/get_tasks",
			get(|| async {
				debug!("Get tasks");
				self.todo_list.get_tasks().unwrap()
			}),
		)
	}

	pub fn get_router(&'static self) -> Router {
		Router::new()
			.merge(self.new_task())
			.merge(self.get_tasks())
			.merge(self.remove_task())
			.merge(self.change_task())
			.merge(self.mark_completed())
	}
}

#[derive(Deserialize)]
struct NewTaskPayload {
	content: String,
}

#[derive(Deserialize)]
struct TaskIDPayload {
	id: usize,
}

#[derive(Deserialize)]
struct ChangeTaskPayload {
	id: usize,
	content: String,
}
