use crate::types::{Error, Params, Value};
use crate::BoxFuture;
use std::fmt;
use std::future::Future;
use std::sync::Arc;

pub trait Metadata: Clone + Send + 'static {}
impl Metadata for () {}
impl<T: Metadata> Metadata for Option<T> {}
impl<T: Metadata> Metadata for Box<T> {}
impl<T: Sync + Send + 'static> Metadata for Arc<T> {}

pub trait WrapFuture<T, E> {
	
	fn into_future(self) -> BoxFuture<Result<T, E>>;
}

impl<T: Send + 'static, E: Send + 'static> WrapFuture<T, E> for Result<T, E> {
	fn into_future(self) -> BoxFuture<Result<T, E>> { panic!("STUB: not implemented") }
}

impl<T, E> WrapFuture<T, E> for BoxFuture<Result<T, E>> {
	fn into_future(self) -> BoxFuture<Result<T, E>> { panic!("STUB: not implemented") }
}

pub trait RpcMethodSync: Send + Sync + 'static {
	
	fn call(&self, params: Params) -> BoxFuture<crate::Result<Value>>;
}

pub trait RpcMethodSimple: Send + Sync + 'static {
	
	type Out: Future<Output = Result<Value, Error>> + Send;
	
	fn call(&self, params: Params) -> Self::Out;
}

pub trait RpcMethod<T: Metadata>: Send + Sync + 'static {
	
	fn call(&self, params: Params, meta: T) -> BoxFuture<crate::Result<Value>>;
}

pub trait RpcNotificationSimple: Send + Sync + 'static {
	
	fn execute(&self, params: Params);
}

pub trait RpcNotification<T: Metadata>: Send + Sync + 'static {
	
	fn execute(&self, params: Params, meta: T);
}

#[derive(Clone)]
pub enum RemoteProcedure<T: Metadata> {
	
	Method(Arc<dyn RpcMethod<T>>),
	
	Notification(Arc<dyn RpcNotification<T>>),
	
	Alias(String),
}

impl<T: Metadata> fmt::Debug for RemoteProcedure<T> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }
}

impl<F: Send + Sync + 'static, X: Send + 'static> RpcMethodSimple for F
where
	F: Fn(Params) -> X,
	X: Future<Output = Result<Value, Error>>,
{
	type Out = X;
	fn call(&self, params: Params) -> Self::Out { panic!("STUB: not implemented") }
}

impl<F: Send + Sync + 'static, X: Send + 'static> RpcMethodSync for F
where
	F: Fn(Params) -> X,
	X: WrapFuture<Value, Error>,
{
	fn call(&self, params: Params) -> BoxFuture<crate::Result<Value>> { panic!("STUB: not implemented") }
}

impl<F: Send + Sync + 'static> RpcNotificationSimple for F
where
	F: Fn(Params),
{
	fn execute(&self, params: Params) { panic!("STUB: not implemented") }
}

impl<F: Send + Sync + 'static, X: Send + 'static, T> RpcMethod<T> for F
where
	T: Metadata,
	F: Fn(Params, T) -> X,
	X: Future<Output = Result<Value, Error>>,
{
	fn call(&self, params: Params, meta: T) -> BoxFuture<crate::Result<Value>> { panic!("STUB: not implemented") }
}

impl<F: Send + Sync + 'static, T> RpcNotification<T> for F
where
	T: Metadata,
	F: Fn(Params, T),
{
	fn execute(&self, params: Params, meta: T) { panic!("STUB: not implemented") }
}
