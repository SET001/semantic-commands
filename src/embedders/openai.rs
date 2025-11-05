use std::collections::HashMap;

use anyhow::{Result, bail};
use log::debug;
use serde::Deserialize;

use crate::Embedder;

const BASE_URL: &str = "https://api.openai.com/v1/";

#[derive(Deserialize, Debug)]
pub struct ErrorDescription {
	pub message: String,
	// pub r#type: String,
	// pub code: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
	pub error: ErrorDescription,
}

#[derive(Deserialize, Debug)]
pub struct SuccessResponse<D> {
	// object: String,
	data: Vec<D>,
	// model: String,
	// usage: ResponseUsage,
}

#[derive(Deserialize, Debug)]
pub struct EmbeddingResponse {
	// object: String,
	embedding: Vec<f32>,
	// index: u32,
}

#[derive(Deserialize, Debug)]
struct ResponseUsage {
	// prompt_tokens: u32,
	// total_tokens: u32,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OpenAIResponse<D> {
	Success(SuccessResponse<D>),
	Error(ErrorResponse),
}

pub struct OpenAIEmbedder {
	pub token: String,
}

#[async_trait::async_trait]
impl Embedder for OpenAIEmbedder {
	async fn embed(&self, input: &str) -> Result<Vec<f32>> {
		let client = reqwest::Client::new();
		let map: HashMap<&str, &str> = HashMap::from_iter(vec![("input", input), ("model", "text-embedding-ada-002")]);

		debug!("fetching embedding from openai for phrase: {input}...");
		let response = client
			.post(format!("{BASE_URL}/embeddings"))
			.bearer_auth(self.token.clone())
			.json(&map)
			.send()
			.await?
			.json::<OpenAIResponse<EmbeddingResponse>>()
			.await?;
		match response {
			OpenAIResponse::Error(error_response) => {
				bail!(error_response.error.message)
			}
			OpenAIResponse::Success(embdding_response) => {
				let asd = embdding_response.data.first().unwrap();
				Ok(asd.embedding.clone())
			}
		}
	}
}
