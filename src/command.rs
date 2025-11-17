use std::{any::Any, future::Future, pin::Pin, sync::Arc};

type Executor<C> = Box<dyn Fn(Arc<C>) -> Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send>> + Send + Sync>;

pub struct Command<C> {
	pub name: String,
	pub requires_confirmation: bool,
	pub executor: Executor<C>,
}

pub fn async_executor<C, F, Fut, R>(f: F) -> Executor<C>
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
