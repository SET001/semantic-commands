# Semantic Commands

[![Crates.io](https://img.shields.io/crates/v/semantic-commands)](https://crates.io/crates/semantic-commands)
[![Documentation](https://docs.rs/semantic-commands/badge.svg)](https://docs.rs/semantic-commands)
[![License](https://img.shields.io/crates/l/semantic-commands)](LICENSE-MIT)

A lightweight Rust framework for defining and executing semantic commands using text embeddings. Frontend‑agnostic and async‑first: route user phrases to your functions based on semantic similarity. Use it in CLI tools, services, web, or desktop applications.

---

## Features

* Define commands with multiple example phrases.
* Async executors with typed results (downcast at call site).
* Pluggable embeddings (implemented: OpenAI)
* Command recognition based on input similarity.
* Optional caching layer for embeddings (implemented: PostgreSQL).
* Context-aware execution.
* Easy integration with multiple interfaces (CLI, web, API, messaging bots).

---


## Usage

Define Commands

```rust
async fn get_date(_ctx: Arc<()>) -> String {
	"2025-11-05".to_string()
}

let command = Command {
	name: "get_date".to_string(),
	requires_confirmation: false,
	executor: async_executor(get_date),
};
let inputs = vec![
	Input::new("what's the date"),
];
```

Initialize SemanticCommands

```rust
let mut semantic_commands = SemanticCommands::new(
	OpenAIEmbedder,		//	OpenAIEmbedder or implement your own.
	NoCache,			//	PostgresCache |	NoCache or implement your own.
	AppContext			//	define your context which will be available in command executors.
);
semantic_commands.add_command(command, inputs);
```

Execute a Command

```rust
let result = semantic_commands.execute("what is the current BTC price?").await?;
```

The result should be then downcasted to whatever type returned by your executor:

```rust
println!("Date: {:?}", result.downcast::<anyhow::Result<String>>().unwrap().unwrap());
```

---


## Caching Options

| Cache | Speed | Memory | Persistence | Use Case |
|-------|-------|--------|-------------|----------|
| `NoCache` | N/A | None | N/A | Testing, stateless |
| `InMemoryCache` | Fast | Unbounded | No | Services, bots |
| `PostgresCache` | Slow | DB-backed | Yes | Multi-instance |

---

## Features

- `openai` (default) - OpenAI embedding provider
- `in-memory-cache` (default) - Fast in-memory LRU cache based on [moka](https://crates.io/crates/moka)
- `postgres` - PostgreSQL cache backend (implemented with [sqlx](https://crates.io/crates/sqlx))
- `full` - All features enabled

---

## Safety & Privacy

Using remote embedding providers (like OpenAI) sends input text to third‑party services. Do not embed secrets or private data you cannot share.

---

## Extensibility

You can implement:
* A custom `Embedder` (e.g. local model)
* A custom `Cache`

---

## License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
