#![doc = include_str!("../README.md")]

pub mod cache;
mod caches;
mod command;
mod embedder;
pub mod embedders;
pub mod input;
mod semantic_commands;

pub use semantic_commands::SemanticCommands;

#[cfg(feature = "in-memory-cache")]
pub use caches::in_memory::InMemoryCache;
pub use caches::no_cache::NoCache;
#[cfg(feature = "postgres")]
pub use caches::postgres::PostgresCache;

pub use command::{Command, async_executor};
#[cfg(feature = "openai")]
pub use embedders::openai::OpenAIEmbedder;

pub use cache::Cache;
pub use embedder::Embedder;
pub use input::Input;
