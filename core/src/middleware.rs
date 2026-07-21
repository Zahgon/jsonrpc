
use crate::calls::Metadata;
use crate::types::{Call, Output, Request, Response};
use futures_util::future::Either;
use std::future::Future;
use std::pin::Pin;

pub trait Middleware<M: Metadata>: Send + Sync + 'static {
	
	type Future: Future<Output = Option<Response>> + Send + 'static;

	type CallFuture: Future<Output = Option<Output>> + Send + 'static;

	fn on_request<F, X>(&self, request: Request, meta: M, next: F) -> Either<Self::Future, X>
	where
		F: Fn(Request, M) -> X + Send + Sync,
		X: Future<Output = Option<Response>> + Send + 'static,
	{ panic!("STUB: not implemented") }

	fn on_call<F, X>(&self, call: Call, meta: M, next: F) -> Either<Self::CallFuture, X>
	where
		F: Fn(Call, M) -> X + Send + Sync,
		X: Future<Output = Option<Output>> + Send + 'static,
	{ panic!("STUB: not implemented") }
}

pub type NoopFuture = Pin<Box<dyn Future<Output = Option<Response>> + Send>>;

pub type NoopCallFuture = Pin<Box<dyn Future<Output = Option<Output>> + Send>>;

#[derive(Clone, Debug, Default)]
pub struct Noop;
impl<M: Metadata> Middleware<M> for Noop {
	type Future = NoopFuture;
	type CallFuture = NoopCallFuture;
}

impl<M: Metadata, A: Middleware<M>, B: Middleware<M>> Middleware<M> for (A, B) {
	type Future = Either<A::Future, B::Future>;
	type CallFuture = Either<A::CallFuture, B::CallFuture>;

	fn on_request<F, X>(&self, request: Request, meta: M, process: F) -> Either<Self::Future, X>
	where
		F: Fn(Request, M) -> X + Send + Sync,
		X: Future<Output = Option<Response>> + Send + 'static,
	{ panic!("STUB: not implemented") }

	fn on_call<F, X>(&self, call: Call, meta: M, process: F) -> Either<Self::CallFuture, X>
	where
		F: Fn(Call, M) -> X + Send + Sync,
		X: Future<Output = Option<Output>> + Send + 'static,
	{ panic!("STUB: not implemented") }
}

impl<M: Metadata, A: Middleware<M>, B: Middleware<M>, C: Middleware<M>> Middleware<M> for (A, B, C) {
	type Future = Either<A::Future, Either<B::Future, C::Future>>;
	type CallFuture = Either<A::CallFuture, Either<B::CallFuture, C::CallFuture>>;

	fn on_request<F, X>(&self, request: Request, meta: M, process: F) -> Either<Self::Future, X>
	where
		F: Fn(Request, M) -> X + Send + Sync,
		X: Future<Output = Option<Response>> + Send + 'static,
	{ panic!("STUB: not implemented") }

	fn on_call<F, X>(&self, call: Call, meta: M, process: F) -> Either<Self::CallFuture, X>
	where
		F: Fn(Call, M) -> X + Send + Sync,
		X: Future<Output = Option<Output>> + Send + 'static,
	{ panic!("STUB: not implemented") }
}

impl<M: Metadata, A: Middleware<M>, B: Middleware<M>, C: Middleware<M>, D: Middleware<M>> Middleware<M>
	for (A, B, C, D)
{
	type Future = Either<A::Future, Either<B::Future, Either<C::Future, D::Future>>>;
	type CallFuture = Either<A::CallFuture, Either<B::CallFuture, Either<C::CallFuture, D::CallFuture>>>;

	fn on_request<F, X>(&self, request: Request, meta: M, process: F) -> Either<Self::Future, X>
	where
		F: Fn(Request, M) -> X + Send + Sync,
		X: Future<Output = Option<Response>> + Send + 'static,
	{ panic!("STUB: not implemented") }

	fn on_call<F, X>(&self, call: Call, meta: M, process: F) -> Either<Self::CallFuture, X>
	where
		F: Fn(Call, M) -> X + Send + Sync,
		X: Future<Output = Option<Output>> + Send + 'static,
	{ panic!("STUB: not implemented") }
}

#[inline(always)]
fn repack<A, B, X>(result: Either<A, Either<B, X>>) -> Either<Either<A, B>, X> { panic!("STUB: not implemented") }
