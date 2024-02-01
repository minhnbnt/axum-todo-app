mod error;
mod todo_list;

use axum::extract::Path;
use axum::response::Html;
use axum::routing::get;
use axum::Router;

use tokio::io;
use tokio::net::TcpListener;

use tracing::info;

use todo_list::RAMTodoList;

#[tokio::main]
async fn main() -> io::Result<()> {
	tracing_subscriber::fmt::init();

	let app = Router::new()
		.nest("/todo", todo_list::get_router(RAMTodoList::new()))
		.route("/", get(|| async { "Hello, World!" }))
		.route("/hello2/:name", get(hello_with_name));

	let listener = TcpListener::bind("127.0.0.1:3333").await?;

	info!("Listening to {}", listener.local_addr()?);
	axum::serve(listener, app).await?;

	Ok(())
}

async fn hello_with_name(Path(name): Path<String>) -> Html<String> {
	Html(format!("Hello, <b>{}</b>!", name))
}
