use anyhow::Result;

use crate::Cache;
pub struct NoCache;

#[async_trait::async_trait]
impl Cache for NoCache {
	async fn get(&self, _: &str) -> Result<Option<Vec<f32>>> {
		Ok(None)
	}
	async fn put(&self, _: &str, _: Vec<f32>) -> Result<()> {
		Ok(())
	}

	async fn init(&self) -> Result<()> {
		Ok(())
	}
}
