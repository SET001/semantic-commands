use anyhow::Result;
use log::debug;

use sqlx::{FromRow, PgPool};

use crate::Cache;

const INPUT_TABLE_NAME: &str = "input";

pub struct PostgresCache {
	pub connection: PgPool,
}

#[derive(Debug, FromRow)]
pub struct PgInput {
	embedding: Vec<f32>,
	// text: String,
}

#[async_trait::async_trait]
impl Cache for PostgresCache {
	async fn init(&self) -> Result<()> {
		debug!("initializing Postgres storage...");
		let queries = vec![format!(
			r#"
				CREATE TABLE IF NOT EXISTS {INPUT_TABLE_NAME}(
					id SERIAL PRIMARY KEY,
					text text NOT NULL UNIQUE,
					embedding REAL[] NOT NULL,
					created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
			)"#
		)];
		for query in queries {
			sqlx::query(&query).execute(&self.connection).await?;
		}

		Ok(())
	}

	async fn get(&self, input: &str) -> Result<Option<Vec<f32>>> {
		let res = sqlx::query_as::<_, PgInput>(&format!("SELECT * FROM {INPUT_TABLE_NAME} WHERE text = $1"))
			.bind(input)
			.fetch_optional(&self.connection)
			.await?;
		Ok(res.map(|record| record.embedding))
	}

	async fn put(&self, input: &str, embedding: Vec<f32>) -> Result<()> {
		sqlx::query(&format!("INSERT INTO {INPUT_TABLE_NAME}(text, embedding) VALUES($1, $2)"))
			.bind(input)
			.bind(embedding)
			.execute(&self.connection)
			.await?;
		Ok(())
	}
}
