use std::sync::Arc;
use std::{any::Any, pin::Pin};

use ai_bot::{Command, Input};

#[derive(serde::Deserialize)]
struct BinancePriceResponse {
	// symbol: String,
	price: String,
}

fn async_executor<C, F, Fut, R>(
	f: F,
) -> Box<dyn Fn(Arc<C>) -> Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send>> + Send + Sync>
where
	F: Fn(Arc<C>) -> Fut + Send + Sync + 'static,
	Fut: Future<Output = R> + Send + 'static,
	R: Any + Send + 'static,
{
	Box::new(move |ctx: Arc<C>| {
		let fut = f(ctx);
		Box::pin(async move { Box::new(fut.await) as Box<dyn Any + Send> })
	})
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
