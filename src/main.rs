use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

use tokio::io;
use tokio::net::TcpListener;

use tracing::info;

mod todo_list;
use todo_list::TodoListRouter;

lazy_static::lazy_static! {
	static ref TODO_LIST_ROUTER: TodoListRouter = TodoListRouter::new();
}

#[tokio::main]
async fn main() -> io::Result<()> {
	let addr = "127.0.0.1:3000";

	tracing_subscriber::fmt::init();

	let app = Router::new()
		.nest("/todo", TODO_LIST_ROUTER.get_router())
		.route("/", get(|| async { "Hello, World!" }))
		.route("/hello2/:name", get(hello_with_name));

	let listener = TcpListener::bind(addr).await?;

	info!("Listening to {}", addr);
	axum::serve(listener, app).await?;

	Ok(())
}

async fn hello_with_name(Path(name): Path<String>) -> impl IntoResponse {
	Html(format!("Hello, <b>{}</b>!", name))
}
