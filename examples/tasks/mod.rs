use std::sync::Arc;

use semantic_commands::{Command, Input, async_executor};

#[derive(serde::Deserialize)]
struct BinancePriceResponse {
	// symbol: String,
	price: String,
}

pub async fn get_coin_course<C>(_context: Arc<C>) -> anyhow::Result<String> {
	let price = reqwest::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")
		.await?
		.json::<BinancePriceResponse>()
		.await?;
	Ok(format!("Current BTC price is: {}", price.price))
}

pub async fn get_date<C>(_context: Arc<C>) -> anyhow::Result<String> {
	Ok(format!("Today's date is: {}", chrono::Utc::now()))
}

pub async fn greet<C>(_context: Arc<C>) -> anyhow::Result<String> {
	Ok("Hi there! How can I assist you today?".to_string())
}

pub async fn ping<C>(_context: Arc<C>) -> anyhow::Result<String> {
	Ok("Pong".to_string())
}

pub fn get_commands<C>() -> Vec<(Command<C>, Vec<Input>)>
where
	C: Send + Sync + 'static,
{
	vec![
		(
			Command {
				name: "get_date".to_string(),
				requires_confirmation: false,
				executor: async_executor(get_date),
			},
			vec![Input::new("what is the date today")],
		),
		(
			Command {
				name: "crypto_rate".to_string(),
				requires_confirmation: false,
				executor: async_executor(get_coin_course),
			},
			vec![Input::new("price of bitcoin")],
		),
		(
			Command {
				name: "greet".to_string(),
				requires_confirmation: false,
				executor: async_executor(greet),
			},
			vec![Input::new("hello")],
		),
		(
			Command {
				name: "ping".to_string(),
				requires_confirmation: false,
				executor: async_executor(ping),
			},
			vec![Input::new("ping")],
		),
	]
}
