use axum::{
	error_handling::HandleErrorLayer,
	extract::{Json, Path, State},
	http::{Method, StatusCode},
	response::{ErrorResponse, IntoResponse, Result},
	routing::{delete, get, patch, post},
	BoxError, Router,
};

use tower::{
	timeout::{self, TimeoutLayer},
	ServiceBuilder,
};

use serde::Deserialize;

use std::sync::Arc;
use std::time::Duration;

use super::{Task, TodoList};

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

	let service = ServiceBuilder::new()
		.layer(cors_layer)
		.layer(HandleErrorLayer::new(|err: BoxError| async move {
			if err.is::<timeout::error::Elapsed>() {
				StatusCode::REQUEST_TIMEOUT
			} else {
				StatusCode::INTERNAL_SERVER_ERROR
			}
		}))
		.layer(TimeoutLayer::new(Duration::from_secs(10)));

	Router::new()
		.route("/", post(new_task).put(change_task))
		.route("/tasks", get(get_tasks))
		.route("/delete/:id", delete(remove_task))
		.route("/complete/:id", patch(mark_completed))
		.with_state(Arc::new(backend))
		.layer(service)
}

async fn get_tasks(list: State<Arc<impl TodoList>>) -> Result<(StatusCode, Json<Vec<Task>>)> {
	match list.get_tasks().await {
		Ok(response) => Ok::<_, ErrorResponse>((StatusCode::OK, response)),
		Err(e) => Err(e.to_string().into()),
	}
}

async fn new_task(
	list: State<Arc<impl TodoList>>,
	Json(request): Json<NewTaskPayload>,
) -> Result<(StatusCode, &'static str)> {
	match list.new_task(request.content).await {
		Ok(()) => Ok((StatusCode::CREATED, "Task created")),
		Err(e) => Err(e.to_string().into()),
	}
}

async fn change_task(
	list: State<Arc<impl TodoList>>,
	Json(request): Json<ChangeTaskPayload>,
) -> Result<(StatusCode, &'static str)> {
	match list.change_task(request.id, request.content).await {
		Ok(_) => Ok((StatusCode::OK, "Task changed")),
		Err(e) => Err(e.to_string().into()),
	}
}

async fn remove_task(id: Path<u32>, list: State<Arc<impl TodoList>>) -> Result<impl IntoResponse> {
	match list.remove_task(*id).await {
		Ok(()) => Ok((StatusCode::OK, "Task removed")),
		Err(e) => Err(e.to_string().into()),
	}
}

async fn mark_completed(
	id: Path<u32>,
	list: State<Arc<impl TodoList>>,
) -> Result<impl IntoResponse> {
	match list.mark_completed(*id).await {
		Ok(()) => Ok((StatusCode::OK, "Task marked completed")),
		Err(e) => Err(e.to_string().into()),
	}
}

#[derive(Deserialize)]
struct NewTaskPayload {
	content: String,
}

#[derive(Deserialize)]
struct ChangeTaskPayload {
	id: u32,
	content: String,
}
