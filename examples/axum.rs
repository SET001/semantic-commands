mod tasks;

use std::sync::Arc;

use anyhow::Result;
use axum::{
	Router,
	extract::{Path, State},
	routing::get,
};
use semantic_commands::{InMemoryCache, OpenAIEmbedder, SemanticCommands};
use tokio::sync::Mutex;

pub struct CommandsContext;

#[derive(Clone)]
pub struct AppState {
	commands: Arc<Mutex<SemanticCommands<OpenAIEmbedder, InMemoryCache, CommandsContext>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenv::dotenv().ok();
	env_logger::init();

	let embedder = OpenAIEmbedder {
		token: std::env::var("OPENAI_KEY")?,
	};

	let mut semantic_commands = SemanticCommands::new(embedder, InMemoryCache::default(), CommandsContext {});
	semantic_commands.add_commands(tasks::get_commands());
	semantic_commands.init().await?;

	let state = AppState {
		commands: Arc::new(Mutex::new(semantic_commands)),
	};

	let app = Router::new()
		.route("/command/{command}", get(execute_command))
		.with_state(state);
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();

	Ok(())
}

async fn execute_command(Path(command): Path<String>, State(state): State<AppState>) -> String {
	let mut commands = state.commands.lock().await;
	let res = commands.execute(&command).await.unwrap();
	let output = res.downcast::<anyhow::Result<String>>().unwrap().unwrap();
	output
}
