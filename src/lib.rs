mod caches;
mod embedders;

pub use embedders::openai::OpenAIEmbedder;
use futures::future::{BoxFuture, join_all};
use log::{error, info};

pub use caches::no_cache::NoCache;
#[cfg(feature = "postgres")]
pub use caches::postgres::PostgresCache;

use std::{any::Any, pin::Pin, sync::Arc};

use anyhow::{Result, bail};

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
	let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
	let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
	let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
	dot / (norm_a * norm_b)
}

fn normalize(s: &str) -> String {
	s.to_lowercase()
		.replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), " ")
		.split_whitespace() //	avoid usage tabs, new lines etc
		.collect::<Vec<_>>()
		.join(" ")
		.trim()
		.to_string()
}

#[derive(Default, Debug)]
pub struct Input {
	text: String,
	empedding: Option<Vec<f32>>,
}

impl Input {
	pub fn new(text: &str) -> Self {
		Self {
			text: normalize(text),
			empedding: None,
		}
	}
}
type Executor<C> = Box<dyn Fn(Arc<C>) -> Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send>> + Send + Sync>;

pub struct Command<C> {
	pub name: String,
	pub requires_confirmation: bool,
	pub executor: Executor<C>,
}

#[async_trait::async_trait]
pub trait Cache {
	async fn get(&self, input: &str) -> Result<Option<Vec<f32>>>;
	async fn put(&self, input: &str, embedding: Vec<f32>) -> Result<()>;
	async fn init(&self) -> Result<()>;
}

#[async_trait::async_trait]
pub trait Embedder {
	async fn embed(&self, input: &str) -> Result<Vec<f32>>;
}

pub struct SemanticCommands<E: Embedder, Ch: Cache, C> {
	embedder: Arc<E>,
	cache: Arc<Ch>,
	context: Arc<C>,
	entries: Vec<(Vec<Input>, Command<C>)>,
}

impl<E: Embedder, Ch: Cache, C> SemanticCommands<E, Ch, C> {
	pub async fn get_embedding(&self, input: &str) -> Result<Vec<f32>> {
		match self.cache.get(input).await? {
			Some(embedding) => Ok(embedding),
			None => {
				info!("embedding not found in cache, generating new one");
				let embedding = self.embedder.as_ref().embed(input).await?;
				self.cache.put(input, embedding.clone()).await?;
				Ok(embedding)
			}
		}
	}

	pub fn new(embedder: E, cache: Ch, context: C) -> Self {
		Self {
			embedder: Arc::new(embedder),
			cache: Arc::new(cache),
			context: Arc::new(context),
			entries: vec![],
		}
	}

	async fn find_similar(&mut self, embedding: Vec<f32>, threshold: f32) -> Result<Option<(&Input, &Command<C>)>> {
		// Pre-calculate all missing embeddings in one batch
		let missing_embeddings: Vec<_> = self
			.entries
			.iter()
			.flat_map(|(inputs, _)| inputs)
			.filter(|input| input.empedding.is_none())
			.map(|input| input.text.clone())
			.collect();

		// Get all embeddings in parallel using existing get_embedding method
		let embeddings: Vec<_> = join_all(missing_embeddings.iter().map(|text| async { self.get_embedding(text).await }))
			.await
			.into_iter()
			.filter_map(Result::ok)
			.collect();

		// Update inputs with new embeddings
		let mut emb_iter = embeddings.into_iter();
		for (inputs, _) in &mut self.entries {
			for input in inputs {
				if input.empedding.is_none() {
					input.empedding = emb_iter.next();
				}
			}
		}

		let res = self
			.entries
			.iter()
			.flat_map(|(inputs, command)| {
				let emb = embedding.clone();
				inputs.iter().filter_map(move |input| {
					let similarity = cosine_similarity(&emb, input.empedding.as_ref().unwrap());
					(similarity >= threshold).then_some((similarity, input, command))
				})
			})
			.collect::<Vec<_>>();

		Ok(res
			.into_iter()
			.max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
			.map(|(_similarity, input, command)| (input, command)))
	}

	pub async fn execute(&mut self, input: &str) -> Result<Box<dyn Any + Send>> {
		let input_embedding = self.get_embedding(input).await?;
		let context = self.context.clone();
		let similar = self.find_similar(input_embedding, 0.2).await?;
		match similar {
			Some((_input, command)) => {
				info!("command recognized as: {:?}", command.name);
				let result = (command.executor)(context).await;
				return Ok(result);
			}
			None => {
				bail!("no similar command found");
			}
		}
	}

	pub fn add_command(&mut self, command: Command<C>, inputs: Vec<Input>) -> &mut Self {
		self.entries.push((inputs, command));
		self
	}

	pub fn add_commands(&mut self, commands: Vec<(Command<C>, Vec<Input>)>) -> &mut Self {
		commands.into_iter().for_each(|(command, inputs)| {
			self.entries.push((inputs, command));
		});
		self
	}

	pub async fn init(&mut self) -> Result<()> {
		self.cache.init().await?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_normalize() {
		let input = "Hello, World! This is a Test.";
		let expected = "hello world this is a test";
		assert_eq!(normalize(input), expected);
	}

	#[test]
	fn test_cosine_similarity() {
		let a = vec![1.0, 0.0, 0.0];
		let b = vec![0.0, 1.0, 0.0];
		let c = vec![1.0, 0.0, 0.0];
		assert!((cosine_similarity(&a, &b) - 0.0).abs() < 1e-6);
		assert!((cosine_similarity(&a, &c) - 1.0).abs() < 1e-6);
	}
}
