# Semantic Commands

[![Crates.io](https://img.shields.io/crates/v/semantic-commands)](https://crates.io/crates/semantic-commands)

A lightweight Rust framework for defining and executing semantic commands using text embeddings. Frontend‑agnostic and async‑first: route user phrases to your functions based on semantic similarity. Use it from CLI apps, web or descktop apps.

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

### Define Commands

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

### Initialize SemanticCommands

```rust
let mut semantic_commands = SemanticCommands::new(
	OpenAIEmbedder,	//	OpenAIEmbedder or implemnent your own.
	NoCache,				//	PostgresCache |	NoCache or implemnent your own.
	AppContext			//	define your context wich will be available in command executors.
);
semantic_commands.add_command(command, inputs);
```

### Execute a Command

```rust
let result = semantic_commands.execute("what is the current BTC price?").await?;
```

The result should be then downcasted to whatever type returned by your executor:

```rust
println!("Date: {:?}", result.downcast::<anyhow::Result<String>>().unwrap().unwrap());
```

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


## Safety & Privacy

Using remote embedding providers (like OpenAI) sends input text to third‑party services. Do not embed secrets or private data you cannot share.

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
