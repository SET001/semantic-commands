use std::sync::Arc;

use ai_bot::{Command, Input};

#[derive(serde::Deserialize)]
struct BinancePriceResponse {
	// symbol: String,
	price: String,
}

pub async fn get_coin_course<C>(_context: Arc<C>) -> anyhow::Result<()> {
	let price = reqwest::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")
		.await?
		.json::<BinancePriceResponse>()
		.await?;
	println!("Current BTC price is: {}", price.price);
	Ok(())
}

pub async fn get_date<C>(_context: Arc<C>) -> anyhow::Result<()> {
	println!("Today's date is: {}", chrono::Utc::now());
	Ok(())
}

pub async fn greet<C>(_context: Arc<C>) -> anyhow::Result<()> {
	println!("Hi there! How can I assist you today?");
	Ok(())
}

pub async fn ping<C>(_context: Arc<C>) -> anyhow::Result<()> {
	println!("Pong");
	Ok(())
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
				executor: Box::new(|ctx| Box::pin(async move { get_date(ctx).await })),
			},
			vec![Input::new("what is the date today")],
		),
		(
			Command {
				name: "crypto_rate".to_string(),
				requires_confirmation: false,
				executor: Box::new(|ctx| Box::pin(async move { get_coin_course(ctx).await })),
			},
			vec![Input::new("price of {coin")],
		),
		(
			Command {
				name: "greet".to_string(),
				requires_confirmation: false,
				executor: Box::new(|ctx| Box::pin(async move { greet(ctx).await })),
			},
			vec![Input::new("hello")],
		),
		(
			Command {
				name: "ping".to_string(),
				requires_confirmation: false,
				executor: Box::new(|ctx| Box::pin(async move { ping(ctx).await })),
			},
			vec![Input::new("ping")],
		),
	]
}
