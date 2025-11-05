use std::{error::Error, str::FromStr};

use ai_bot::{OpenAIEmbedder, PostgresCache, SemanticCommands};
use anyhow::Context;
use clap::Parser;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
mod tasks;

#[derive(Parser)]
struct Cli {
	input: String,
}

use std::env;

fn env_var<T>(key: &str) -> anyhow::Result<T>
where
	T: FromStr,
	<T as FromStr>::Err: Error + Send + Sync + 'static,
{
	std::env::var(key)
		.with_context(|| format!("env variable {key} not found"))?
		.parse()
		.with_context(|| format!("failed to parse env variable {key}"))
}

#[derive(Debug, Clone)]
pub struct DBConfig {
	pub host: String,
	pub port: u32,
	pub username: String,
	pub password: String,
	pub database: String,
}

pub struct AppContext {
	pub db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
	dotenv::dotenv().ok();
	env_logger::builder().init();

	let args = Cli::parse();

	let openai_token = env::var("OPENAI_KEY")?;

	let db_host = env_var("DB_HOST").unwrap_or("localhost".to_string());
	let db_port = env_var("DB_PORT").unwrap_or(5432);
	let db_name = env_var::<String>("DB_NAME")?;
	let db_user = env_var::<String>("DB_USER")?;
	let db_password = env_var::<String>("DB_PASSWORD")?;
	let connect_options = PgConnectOptions::new()
		.host(&db_host)
		.port(db_port)
		.database(&db_name)
		.username(&db_user)
		.password(&db_password);
	let pool = PgPoolOptions::new().max_connections(1).connect_with(connect_options).await?;
	let mut semantic_commands = SemanticCommands::new(
		OpenAIEmbedder { token: openai_token },
		PostgresCache {
			connection: pool.clone(),
		},
		AppContext { db: pool },
	);
	semantic_commands.init().await?;

	let res = semantic_commands
		.add_commands(tasks::get_commands())
		.execute(&args.input)
		.await?
		.downcast::<anyhow::Result<String>>()
		.unwrap()
		.unwrap();
	println!("{:?}", res);
	Ok(())
}
