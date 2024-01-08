use crate::todo_list::TodoList;

use axum::http::StatusCode;
use axum::routing::get;
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

	pub fn get_router(&'static self) -> Router {
		let new_task_func = move |Json(payload): Json<NewTaskPayload>| async move {
			debug!("New task: {}", payload.content);
			self.todo_list.new_task(payload.content);
			(StatusCode::CREATED, "Task created")
		};

		let remove_task_func = move |Json(payload): Json<TaskIDPayload>| async move {
			debug!("Remove task: {}", payload.id);
			self.todo_list.remove_task(payload.id);
			(StatusCode::OK, "Task removed")
		};

		let change_task_func = move |Json(payload): Json<ChangeTaskPayload>| async move {
			debug!("Change task: {}", payload.id);
			self.todo_list.change_task(payload.id, payload.content);
			(StatusCode::OK, "Task changed")
		};

		let mark_completed_func = move |Json(payload): Json<TaskIDPayload>| async move {
			debug!("Mark completed: {}", payload.id);
			self.todo_list.mark_completed(payload.id);
			(StatusCode::OK, "Task marked completed")
		};

		let get_task_func = move || async move {
			debug!("Get tasks");
			self.todo_list.get_tasks().unwrap()
		};

		Router::new().route(
			"/",
			get(get_task_func)
				.post(new_task_func)
				.patch(change_task_func)
				.delete(remove_task_func)
				.put(mark_completed_func),
		)
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
