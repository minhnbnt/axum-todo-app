use crate::todo_list::TodoList;

use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};

use serde::Deserialize;

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
			self.todo_list.new_task(payload.content);
			(StatusCode::CREATED, "Task created")
		};

		Router::new().route("/new/:content", post(function))
	}

	fn remove_task(&'static self) -> Router {
		let function = move |Json(payload): Json<TaskIDPayload>| async move {
			self.todo_list.remove_task(payload.id);
		};
		Router::new().route("/remove/:id", post(function))
	}

	fn change_task(&'static self) -> Router {
		let function = move |Json(payload): Json<ChangeTaskPayload>| async move {
			self.todo_list.change_task(payload.id, payload.content);
		};
		Router::new().route("/change/:id/:content", post(function))
	}

	fn mark_completed(&'static self) -> Router {
		let function = move |Json(payload): Json<TaskIDPayload>| async move {
			self.todo_list.mark_completed(payload.id);
		};
		Router::new().route("/mark_completed/:id", post(function))
	}

	fn get_tasks(&'static self) -> Router {
		let function = move || async move {
			match self.todo_list.get_tasks() {
				Ok(tasks) => (StatusCode::OK, Json(tasks)),
				Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())),
			}
		};
		Router::new().route("/get_tasks", get(function))
	}

	pub fn get_router(&'static self) -> Router {
		Router::new()
			.merge(self.new_task())
			.merge(self.remove_task())
			.merge(self.change_task())
			.merge(self.mark_completed())
			.merge(self.get_tasks())
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
