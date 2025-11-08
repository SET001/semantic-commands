use anyhow::Result;
use semantic_commands::{Command, InMemoryCache, Input, NoCache, OpenAIEmbedder, SemanticCommands, async_executor};
use std::sync::Arc;

#[derive(Clone)]
struct AppContext;

async fn get_date(_ctx: Arc<AppContext>) -> Result<String> {
	Ok(chrono::Utc::now().date_naive().format("%Y-%m-%d").to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenv::dotenv().ok();
	let embedder = OpenAIEmbedder {
		token: std::env::var("OPENAI_KEY")?,
	};

	let cmd = Command {
		name: "get_date".to_string(),
		requires_confirmation: false,
		executor: async_executor(get_date),
	};

	let inputs = vec![
		Input::new("what's the date"),
		Input::new("tell me the date"),
		Input::new("current date"),
	];

	let date = SemanticCommands::new(embedder, InMemoryCache::default(), AppContext)
		.add_command(cmd, inputs)
		.init()
		.await?
		.execute("tell me the date please")
		.await?
		.downcast::<anyhow::Result<String>>()
		.unwrap()
		.unwrap();

	println!("Date: {date}");
	Ok(())
}
