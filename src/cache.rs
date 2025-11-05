use anyhow::Result;

#[async_trait::async_trait]
pub trait Cache {
	async fn get(&self, input: &str) -> Result<Option<Vec<f32>>>;
	async fn put(&self, input: &str, embedding: Vec<f32>) -> Result<()>;
	async fn init(&self) -> Result<()>;
}
