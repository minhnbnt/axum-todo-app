use crate::todo_list::TodoList;

use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post};
use axum::Router;

use serde::Deserialize;

use std::sync::Arc;

pub fn get_router() -> Router {
	Router::new()
		.route("/", post(new_task).put(change_task))
		.route("/tasks", get(get_tasks))
		.route("/delete/:id", delete(remove_task))
		.route("/complete/:id", patch(mark_completed))
		.with_state(Arc::new(TodoList::new()))
}

async fn get_tasks(State(list): State<Arc<TodoList>>) -> impl IntoResponse {
	(StatusCode::OK, list.get_tasks().unwrap())
}

async fn new_task(
	State(list): State<Arc<TodoList>>,
	Json(request): Json<NewTaskPayload>,
) -> impl IntoResponse {
	list.new_task(request.content);
	(StatusCode::CREATED, "Task created")
}

async fn change_task(
	State(list): State<Arc<TodoList>>,
	Json(request): Json<ChangeTaskPayload>,
) -> impl IntoResponse {
	list.change_task(request.id, request.content);
	(StatusCode::OK, "Task changed")
}

async fn remove_task(
	Path(id): Path<usize>,
	State(list): State<Arc<TodoList>>,
) -> impl IntoResponse {
	list.remove_task(id);
	(StatusCode::OK, "Task removed")
}

async fn mark_completed(
	Path(id): Path<usize>,
	State(list): State<Arc<TodoList>>,
) -> impl IntoResponse {
	list.mark_completed(id);
	(StatusCode::OK, "Task marked completed")
}

#[derive(Deserialize)]
struct NewTaskPayload {
	content: String,
}

#[derive(Deserialize)]
struct ChangeTaskPayload {
	id: usize,
	content: String,
}
