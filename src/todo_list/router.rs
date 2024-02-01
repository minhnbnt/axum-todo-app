use axum::{
	extract::{Json, Path, State},
	http::{Method, StatusCode},
	response::IntoResponse,
	routing::{delete, get, patch, post},
	Router,
};

use crate::error::AppError;

use serde::Deserialize;

use std::sync::Arc;

use super::TodoList;

use tower_http::cors::{Any, CorsLayer};

pub fn get_router<Backend>(backend: Backend) -> Router
where
	Backend: 'static + TodoList + Send + Sync, {
	let cors_layer = CorsLayer::new()
		.allow_methods([
			Method::DELETE,
			Method::GET,
			Method::PATCH,
			Method::POST,
			Method::PUT,
		])
		.allow_origin(Any);

	Router::new()
		.route("/", post(new_task).put(change_task))
		.route("/tasks", get(get_tasks))
		.route("/delete/:id", delete(remove_task))
		.route("/complete/:id", patch(mark_completed))
		.with_state(Arc::new(backend))
		.layer(cors_layer)
}

async fn get_tasks(State(list): State<Arc<impl TodoList>>) -> Result<impl IntoResponse, AppError> {
	Ok((StatusCode::OK, list.get_tasks()?))
}

async fn new_task(
	State(list): State<Arc<impl TodoList>>,
	Json(request): Json<NewTaskPayload>,
) -> Result<impl IntoResponse, AppError> {
	list.new_task(request.content)?;
	Ok((StatusCode::CREATED, "Task created"))
}

async fn change_task(
	State(list): State<Arc<impl TodoList>>,
	Json(request): Json<ChangeTaskPayload>,
) -> Result<impl IntoResponse, AppError> {
	list.change_task(request.id, request.content)?;
	Ok((StatusCode::OK, "Task changed"))
}

async fn remove_task(
	Path(id): Path<usize>,
	State(list): State<Arc<impl TodoList>>,
) -> Result<impl IntoResponse, AppError> {
	list.remove_task(id)?;
	Ok((StatusCode::OK, "Task removed"))
}

async fn mark_completed(
	Path(id): Path<usize>,
	State(list): State<Arc<impl TodoList>>,
) -> Result<impl IntoResponse, AppError> {
	list.mark_completed(id)?;
	Ok((StatusCode::OK, "Task marked completed"))
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
