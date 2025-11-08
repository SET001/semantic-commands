use std::sync::Arc;

use crate::Cache;
use anyhow::Result;
use moka::future::Cache as MokaCache;

pub struct InMemoryCache {
	cache: MokaCache<String, Arc<Vec<f32>>>,
}

impl InMemoryCache {
	/// Unbounded cache (grows indefinitely)
	///
	/// Suitable for:
	/// - Testing/development
	/// - Short-lived processes
	/// - Known input sets
	pub fn unbounded() -> Self {
		Self {
			cache: MokaCache::builder().build(),
		}
	}

	/// Bounded by entry count
	///
	/// Evicts least-recently-used entries when full.
	///
	/// Example: 10,000 entries ≈ 60 MB (at 1536 dims/embedding)
	pub fn with_max_entries(max_entries: u64) -> Self {
		Self {
			cache: MokaCache::builder().max_capacity(max_entries).build(),
		}
	}

	/// Bounded by approximate memory usage
	///
	/// Weights entries by `vector.len() * 4` bytes (f32 size).
	///
	/// Example: 100 MB for ~17,000 embeddings (at 1536 dims)
	pub fn with_max_memory_mb(mb: u64) -> Self {
		let max_bytes = mb * 1024 * 1024;
		Self {
			cache: MokaCache::builder()
				.weigher(|_key: &String, value: &Arc<Vec<f32>>| (value.len() * 4) as u32)
				.max_capacity(max_bytes)
				.build(),
		}
	}
}

impl Default for InMemoryCache {
	/// Default: bounded to 10,000 entries (~60 MB)
	fn default() -> Self {
		Self::with_max_entries(10_000)
	}
}

#[async_trait::async_trait]
impl Cache for InMemoryCache {
	async fn get(&self, input: &str) -> Result<Option<Vec<f32>>> {
		Ok(self.cache.get(input).await.map(|arc| (*arc).clone()))
	}

	async fn put(&self, input: &str, embedding: Vec<f32>) -> Result<()> {
		self.cache.insert(input.to_string(), Arc::new(embedding)).await;
		Ok(())
	}

	async fn init(&self) -> Result<()> {
		Ok(())
	}
}
