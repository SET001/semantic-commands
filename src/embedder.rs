use anyhow::Result;

#[async_trait::async_trait]
pub trait Embedder {
	async fn embed(&self, input: &str) -> Result<Vec<f32>>;
}
