use std::collections::{
	hash_map::{IntoIter, Iter},
	HashMap,
};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::Arc;

use futures_util::{self, future, FutureExt};

use crate::calls::{
	Metadata, RemoteProcedure, RpcMethod, RpcMethodSimple, RpcMethodSync, RpcNotification, RpcNotificationSimple,
};
use crate::middleware::{self, Middleware};
use crate::types::{Call, Output, Request, Response};
use crate::types::{Error, ErrorCode, Version};

pub type FutureResponse = Pin<Box<dyn Future<Output = Option<Response>> + Send>>;

pub type FutureOutput = Pin<Box<dyn Future<Output = Option<Output>> + Send>>;

pub type FutureResult<F, G> = future::Map<
	future::Either<future::Ready<Option<Response>>, FutureRpcResult<F, G>>,
	fn(Option<Response>) -> Option<String>,
>;

pub type FutureRpcOutput<F> = future::Either<F, future::Either<FutureOutput, future::Ready<Option<Output>>>>;

pub type FutureRpcResult<F, G> = future::Either<
	F,
	future::Either<
		future::Map<FutureRpcOutput<G>, fn(Option<Output>) -> Option<Response>>,
		future::Map<future::JoinAll<FutureRpcOutput<G>>, fn(Vec<Option<Output>>) -> Option<Response>>,
	>,
>;

#[derive(Debug, Clone, Copy)]
pub enum Compatibility {
	
	V1,
	
	V2,
	
	Both,
}

impl Default for Compatibility {
	fn default() -> Self { panic!("STUB: not implemented") }
}

impl Compatibility {
	fn is_version_valid(self, version: Option<Version>) -> bool { panic!("STUB: not implemented") }

	fn default_version(self) -> Option<Version> { panic!("STUB: not implemented") }
}

#[derive(Clone, Debug)]
pub struct MetaIoHandler<T: Metadata, S: Middleware<T> = middleware::Noop> {
	middleware: S,
	compatibility: Compatibility,
	methods: HashMap<String, RemoteProcedure<T>>,
}

impl<T: Metadata> Default for MetaIoHandler<T> {
	fn default() -> Self { panic!("STUB: not implemented") }
}

impl<T: Metadata, S: Middleware<T>> IntoIterator for MetaIoHandler<T, S> {
	type Item = (String, RemoteProcedure<T>);
	type IntoIter = IntoIter<String, RemoteProcedure<T>>;

	fn into_iter(self) -> Self::IntoIter { panic!("STUB: not implemented") }
}

impl<'a, T: Metadata, S: Middleware<T>> IntoIterator for &'a MetaIoHandler<T, S> {
	type Item = (&'a String, &'a RemoteProcedure<T>);
	type IntoIter = Iter<'a, String, RemoteProcedure<T>>;

	fn into_iter(self) -> Self::IntoIter { panic!("STUB: not implemented") }
}

impl<T: Metadata> MetaIoHandler<T> {
	
	pub fn with_compatibility(compatibility: Compatibility) -> Self { panic!("STUB: not implemented") }
}

impl<T: Metadata, S: Middleware<T>> MetaIoHandler<T, S> {
	
	pub fn new(compatibility: Compatibility, middleware: S) -> Self { panic!("STUB: not implemented") }

	pub fn with_middleware(middleware: S) -> Self { panic!("STUB: not implemented") }

	pub fn add_alias(&mut self, alias: &str, other: &str) { panic!("STUB: not implemented") }

	pub fn add_sync_method<F>(&mut self, name: &str, method: F)
	where
		F: RpcMethodSync,
	{ panic!("STUB: not implemented") }

	pub fn add_method<F>(&mut self, name: &str, method: F)
	where
		F: RpcMethodSimple,
	{ panic!("STUB: not implemented") }

	pub fn add_notification<F>(&mut self, name: &str, notification: F)
	where
		F: RpcNotificationSimple,
	{ panic!("STUB: not implemented") }

	pub fn add_method_with_meta<F>(&mut self, name: &str, method: F)
	where
		F: RpcMethod<T>,
	{ panic!("STUB: not implemented") }

	pub fn add_notification_with_meta<F>(&mut self, name: &str, notification: F)
	where
		F: RpcNotification<T>,
	{ panic!("STUB: not implemented") }

	pub fn extend_with<F>(&mut self, methods: F)
	where
		F: IntoIterator<Item = (String, RemoteProcedure<T>)>,
	{ panic!("STUB: not implemented") }

	#[cfg(feature = "futures-executor")]
	pub fn handle_request_sync(&self, request: &str, meta: T) -> Option<String> { panic!("STUB: not implemented") }

	pub fn handle_request(&self, request: &str, meta: T) -> FutureResult<S::Future, S::CallFuture> { panic!("STUB: not implemented") }

	pub fn handle_rpc_request(&self, request: Request, meta: T) -> FutureRpcResult<S::Future, S::CallFuture> { panic!("STUB: not implemented") }

	pub fn handle_call(&self, call: Call, meta: T) -> FutureRpcOutput<S::CallFuture> { panic!("STUB: not implemented") }

	pub fn iter(&self) -> impl Iterator<Item = (&String, &RemoteProcedure<T>)> {
		self.methods.iter()
	}
}

pub trait IoHandlerExtension<M: Metadata = ()> {
	
	fn augment<S: Middleware<M>>(self, handler: &mut MetaIoHandler<M, S>);
}

macro_rules! impl_io_handler_extension {
	($( $x:ident, )*) => {
		impl<M, $( $x, )*> IoHandlerExtension<M> for ($( $x, )*) where
			M: Metadata,
			$(
				$x: IoHandlerExtension<M>,
			)*
			{
				#[allow(unused)]
				fn augment<S: Middleware<M>>(self, handler: &mut MetaIoHandler<M, S>) {
					#[allow(non_snake_case)]
					let (
						$( $x, )*
					) = self;
					$(
						$x.augment(handler);
					)*
				}
			}
	}
}

impl_io_handler_extension!();
impl_io_handler_extension!(A,);
impl_io_handler_extension!(A, B,);
impl_io_handler_extension!(A, B, C,);
impl_io_handler_extension!(A, B, C, D,);
impl_io_handler_extension!(A, B, C, D, E,);
impl_io_handler_extension!(A, B, C, D, E, F,);
impl_io_handler_extension!(A, B, C, D, E, F, G,);
impl_io_handler_extension!(A, B, C, D, E, F, G, H,);
impl_io_handler_extension!(A, B, C, D, E, F, G, H, I,);
impl_io_handler_extension!(A, B, C, D, E, F, G, H, I, J,);
impl_io_handler_extension!(A, B, C, D, E, F, G, H, I, J, K,);
impl_io_handler_extension!(A, B, C, D, E, F, G, H, I, J, K, L,);

impl<M: Metadata> IoHandlerExtension<M> for Vec<(String, RemoteProcedure<M>)> {
	fn augment<S: Middleware<M>>(self, handler: &mut MetaIoHandler<M, S>) { panic!("STUB: not implemented") }
}

impl<M: Metadata> IoHandlerExtension<M> for HashMap<String, RemoteProcedure<M>> {
	fn augment<S: Middleware<M>>(self, handler: &mut MetaIoHandler<M, S>) { panic!("STUB: not implemented") }
}

impl<M: Metadata, S2: Middleware<M>> IoHandlerExtension<M> for MetaIoHandler<M, S2> {
	fn augment<S: Middleware<M>>(self, handler: &mut MetaIoHandler<M, S>) { panic!("STUB: not implemented") }
}

impl<M: Metadata, T: IoHandlerExtension<M>> IoHandlerExtension<M> for Option<T> {
	fn augment<S: Middleware<M>>(self, handler: &mut MetaIoHandler<M, S>) { panic!("STUB: not implemented") }
}

#[derive(Clone, Debug, Default)]
pub struct IoHandler<M: Metadata = ()>(MetaIoHandler<M>);

impl<T: Metadata> IntoIterator for IoHandler<T> {
	type Item = <MetaIoHandler<T> as IntoIterator>::Item;
	type IntoIter = <MetaIoHandler<T> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter { panic!("STUB: not implemented") }
}

impl IoHandler {
	
	pub fn new() -> Self { panic!("STUB: not implemented") }

	pub fn with_compatibility(compatibility: Compatibility) -> Self { panic!("STUB: not implemented") }
}

impl<M: Metadata + Default> IoHandler<M> {
	
	pub fn handle_request(&self, request: &str) -> FutureResult<FutureResponse, FutureOutput> { panic!("STUB: not implemented") }

	pub fn handle_rpc_request(&self, request: Request) -> FutureRpcResult<FutureResponse, FutureOutput> { panic!("STUB: not implemented") }

	pub fn handle_call(&self, call: Call) -> FutureRpcOutput<FutureOutput> { panic!("STUB: not implemented") }

	#[cfg(feature = "futures-executor")]
	pub fn handle_request_sync(&self, request: &str) -> Option<String> { panic!("STUB: not implemented") }
}

impl<M: Metadata> Deref for IoHandler<M> {
	type Target = MetaIoHandler<M>;

	fn deref(&self) -> &Self::Target { panic!("STUB: not implemented") }
}

impl<M: Metadata> DerefMut for IoHandler<M> {
	fn deref_mut(&mut self) -> &mut Self::Target { panic!("STUB: not implemented") }
}

impl From<IoHandler> for MetaIoHandler<()> {
	fn from(io: IoHandler) -> Self { panic!("STUB: not implemented") }
}

impl<M: Metadata> IoHandlerExtension<M> for IoHandler<M> {
	fn augment<S: Middleware<M>>(self, handler: &mut MetaIoHandler<M, S>) { panic!("STUB: not implemented") }
}

fn read_request(request_str: &str) -> Result<Request, Error> { panic!("STUB: not implemented") }

fn write_response(response: Response) -> String { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
	use super::{Compatibility, IoHandler};
	use crate::types::Value;

	#[test]
	fn test_io_handler() {
		let mut io = IoHandler::new();

		io.add_method("say_hello", |_| async { Ok(Value::String("hello".to_string())) });

		let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}"#;
		let response = r#"{"jsonrpc":"2.0","result":"hello","id":1}"#;

		assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
	}

	#[test]
	fn test_io_handler_1dot0() {
		let mut io = IoHandler::with_compatibility(Compatibility::Both);

		io.add_method("say_hello", |_| async { Ok(Value::String("hello".to_string())) });

		let request = r#"{"method": "say_hello", "params": [42, 23], "id": 1}"#;
		let response = r#"{"result":"hello","id":1}"#;

		assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
	}

	#[test]
	fn test_async_io_handler() {
		let mut io = IoHandler::new();

		io.add_method("say_hello", |_| async { Ok(Value::String("hello".to_string())) });

		let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}"#;
		let response = r#"{"jsonrpc":"2.0","result":"hello","id":1}"#;

		assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
	}

	#[test]
	fn test_notification() {
		use std::sync::atomic;
		use std::sync::Arc;

		let mut io = IoHandler::new();

		let called = Arc::new(atomic::AtomicBool::new(false));
		let c = called.clone();
		io.add_notification("say_hello", move |_| {
			c.store(true, atomic::Ordering::SeqCst);
		});
		let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23]}"#;

		assert_eq!(io.handle_request_sync(request), None);
		assert_eq!(called.load(atomic::Ordering::SeqCst), true);
	}

	#[test]
	fn test_method_not_found() {
		let io = IoHandler::new();

		let request = r#"{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23], "id": 1}"#;
		let response = r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":1}"#;

		assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
	}

	#[test]
	fn test_method_alias() {
		let mut io = IoHandler::new();
		io.add_method("say_hello", |_| async { Ok(Value::String("hello".to_string())) });
		io.add_alias("say_hello_alias", "say_hello");

		let request = r#"{"jsonrpc": "2.0", "method": "say_hello_alias", "params": [42, 23], "id": 1}"#;
		let response = r#"{"jsonrpc":"2.0","result":"hello","id":1}"#;

		assert_eq!(io.handle_request_sync(request), Some(response.to_string()));
	}

	#[test]
	fn test_notification_alias() {
		use std::sync::atomic;
		use std::sync::Arc;

		let mut io = IoHandler::new();

		let called = Arc::new(atomic::AtomicBool::new(false));
		let c = called.clone();
		io.add_notification("say_hello", move |_| {
			c.store(true, atomic::Ordering::SeqCst);
		});
		io.add_alias("say_hello_alias", "say_hello");

		let request = r#"{"jsonrpc": "2.0", "method": "say_hello_alias", "params": [42, 23]}"#;
		assert_eq!(io.handle_request_sync(request), None);
		assert_eq!(called.load(atomic::Ordering::SeqCst), true);
	}

	#[test]
	fn test_batch_notification() {
		use std::sync::atomic;
		use std::sync::Arc;

		let mut io = IoHandler::new();

		let called = Arc::new(atomic::AtomicBool::new(false));
		let c = called.clone();
		io.add_notification("say_hello", move |_| {
			c.store(true, atomic::Ordering::SeqCst);
		});

		let request = r#"[{"jsonrpc": "2.0", "method": "say_hello", "params": [42, 23]}]"#;
		assert_eq!(io.handle_request_sync(request), None);
		assert_eq!(called.load(atomic::Ordering::SeqCst), true);
	}

	#[test]
	fn test_send_sync() {
		fn is_send_sync<T>(_obj: T) -> bool
		where
			T: Send + Sync,
		{
			true
		}

		let io = IoHandler::new();

		assert!(is_send_sync(io))
	}

	#[test]
	fn test_extending_by_multiple_delegates() {
		use super::IoHandlerExtension;
		use crate::delegates::IoDelegate;
		use std::sync::Arc;

		struct Test;
		impl Test {
			fn abc(&self, _p: crate::Params) -> crate::BoxFuture<crate::Result<Value>> {
				Box::pin(async { Ok(5.into()) })
			}
		}

		let mut io = IoHandler::new();
		let mut del1 = IoDelegate::new(Arc::new(Test));
		del1.add_method("rpc_test", Test::abc);
		let mut del2 = IoDelegate::new(Arc::new(Test));
		del2.add_method("rpc_test", Test::abc);

		fn augment<X: IoHandlerExtension>(x: X, io: &mut IoHandler) {
			x.augment(io);
		}

		augment((del1, del2), &mut io);
	}
}
