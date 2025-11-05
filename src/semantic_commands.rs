use anyhow::{Result, bail};
use futures::future::join_all;
use log::info;
use std::{any::Any, sync::Arc};

use crate::{Command, cache::Cache, embedder::Embedder, input::Input};

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
	let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
	let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
	let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
	dot / (norm_a * norm_b)
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
					let similarity = cosine_similarity(&emb, input.empedding.as_ref()?);
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
	fn test_cosine_similarity() {
		let a = vec![1.0, 0.0, 0.0];
		let b = vec![0.0, 1.0, 0.0];
		let c = vec![1.0, 0.0, 0.0];
		assert!((cosine_similarity(&a, &b) - 0.0).abs() < 1e-6);
		assert!((cosine_similarity(&a, &c) - 1.0).abs() < 1e-6);
	}
}
