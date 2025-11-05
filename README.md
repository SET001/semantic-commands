# Semantic Commands

[![Crates.io](https://img.shields.io/crates/v/semantic-commands)](https://crates.io/crates/semantic-commands)

A Rust library for defining, managing, and executing semantic commands with asynchronous support and flexible output handling. Designed to be frontend-agnostic, allowing commands to return results that can be consumed in CLI applications, web apps, or API services.

---

## Features

* Define commands with inputs and async executors.
* Flexible output types.
* Command recognition based on input similarity.
* Optional caching layer for embeddings.
* Context-aware execution.
* Easy integration with multiple interfaces (CLI, web, API).

---


## Usage

### Define Commands

```rust
use semantic_commands::{Command, async_executor};
use std::sync::Arc;

async fn get_date(_ctx: Arc<()>) -> String {
    "2025-11-05".to_string()
}

async fn get_coin_course(_ctx: Arc<()>) -> f64 {
    29750.0
}

let commands = vec![
    Command {
        name: "get_date".to_string(),
        requires_confirmation: false,
        executor: async_executor(get_date),
    },
    Command {
        name: "crypto_rate".to_string(),
        requires_confirmation: false,
        executor: async_executor(get_coin_course),
    },
];
```

### Initialize SemanticCommands

```rust
use semantic_commands::SemanticCommands;
use std::sync::Arc;

let embedder = Arc::new(MyEmbedder::new()); // your embedder implementation
let cache = Arc::new(NoCache::new());      // or your caching layer. Use NoCache if no caching needed

let mut semantic_commands = SemanticCommands::new(embedder, cache);
semantic_commands.add_commands(commands);
```

### Execute a Command

```rust
let input = "what is the current BTC price?";
let result = semantic_commands.execute(input).await?;
println!("Result: {:?}", result);
```

The result can be a `serde_json::Value` or any type returned by your executor.

---

## API Overview

### `Command<C>`

Represents a single command.

* `name: String` — name of the command.
* `requires_confirmation: bool` — whether the command needs confirmation.
* `executor: Executor<C>` — async function to execute the command.

### `SemanticCommands<E, Ch, C>`

Manages multiple commands and performs similarity-based matching.

* `add_commands(commands: Vec<Command<C>>) -> &mut Self` — add multiple commands.
* `execute(input: &str) -> Result<serde_json::Value>` — execute the command most similar to input.

### `async_executor` helper

Simplifies creating an executor from an async function:

```rust
async_executor(get_date)
```

---

## Features in Depth

* **Input similarity matching**: Commands are chosen based on vector embeddings and a similarity threshold.
* **Flexible execution**: Executors can return any type.
* **Caching**: Supports pluggable cache layers; you can provide `NoCache` if caching is not needed.
* **Context support**: Pass shared context (`Arc<C>`) to executors for richer command behavior.

---

## License

MIT
