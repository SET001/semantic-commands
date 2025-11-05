use std::sync::Arc;
use std::{any::Any, pin::Pin};

use ai_bot::{Command, Input};

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
				executor: Box::new(|ctx: Arc<C>| -> Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send>> {
					Box::pin(async move { Box::new(get_date(ctx).await) as Box<dyn Any + Send> })
				}),
			},
			vec![Input::new("what is the date today")],
		),
		(
			Command {
				name: "crypto_rate".to_string(),
				requires_confirmation: false,
				executor: Box::new(|ctx: Arc<C>| -> Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send>> {
					Box::pin(async move { Box::new(get_coin_course(ctx).await) as Box<dyn Any + Send> })
				}),
			},
			vec![Input::new("price of {coin")],
		),
		(
			Command {
				name: "greet".to_string(),
				requires_confirmation: false,
				executor: Box::new(|ctx: Arc<C>| -> Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send>> {
					Box::pin(async move { Box::new(greet(ctx).await) as Box<dyn Any + Send> })
				}),
			},
			vec![Input::new("hello")],
		),
		(
			Command {
				name: "ping".to_string(),
				requires_confirmation: false,
				executor: Box::new(|ctx: Arc<C>| -> Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send>> {
					Box::pin(async move { Box::new(ping(ctx).await) as Box<dyn Any + Send> })
				}),
			},
			vec![Input::new("ping")],
		),
	]
}
