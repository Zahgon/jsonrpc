
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use crate::calls::{Metadata, RemoteProcedure, RpcMethod, RpcNotification};
use crate::types::{Error, Params, Value};
use crate::BoxFuture;

struct DelegateAsyncMethod<T, F> {
	delegate: Arc<T>,
	closure: F,
}

impl<T, M, F, I> RpcMethod<M> for DelegateAsyncMethod<T, F>
where
	M: Metadata,
	F: Fn(&T, Params) -> I,
	I: Future<Output = Result<Value, Error>> + Send + 'static,
	T: Send + Sync + 'static,
	F: Send + Sync + 'static,
{
	fn call(&self, params: Params, _meta: M) -> BoxFuture<crate::Result<Value>> { panic!("STUB: not implemented") }
}

struct DelegateMethodWithMeta<T, F> {
	delegate: Arc<T>,
	closure: F,
}

impl<T, M, F, I> RpcMethod<M> for DelegateMethodWithMeta<T, F>
where
	M: Metadata,
	F: Fn(&T, Params, M) -> I,
	I: Future<Output = Result<Value, Error>> + Send + 'static,
	T: Send + Sync + 'static,
	F: Send + Sync + 'static,
{
	fn call(&self, params: Params, meta: M) -> BoxFuture<crate::Result<Value>> { panic!("STUB: not implemented") }
}

struct DelegateNotification<T, F> {
	delegate: Arc<T>,
	closure: F,
}

impl<T, M, F> RpcNotification<M> for DelegateNotification<T, F>
where
	M: Metadata,
	F: Fn(&T, Params) + 'static,
	F: Send + Sync + 'static,
	T: Send + Sync + 'static,
{
	fn execute(&self, params: Params, _meta: M) { panic!("STUB: not implemented") }
}

struct DelegateNotificationWithMeta<T, F> {
	delegate: Arc<T>,
	closure: F,
}

impl<T, M, F> RpcNotification<M> for DelegateNotificationWithMeta<T, F>
where
	M: Metadata,
	F: Fn(&T, Params, M) + 'static,
	F: Send + Sync + 'static,
	T: Send + Sync + 'static,
{
	fn execute(&self, params: Params, meta: M) { panic!("STUB: not implemented") }
}

pub struct IoDelegate<T, M = ()>
where
	T: Send + Sync + 'static,
	M: Metadata,
{
	delegate: Arc<T>,
	methods: HashMap<String, RemoteProcedure<M>>,
}

impl<T, M> IoDelegate<T, M>
where
	T: Send + Sync + 'static,
	M: Metadata,
{
	
	pub fn new(delegate: Arc<T>) -> Self { panic!("STUB: not implemented") }

	pub fn add_alias(&mut self, from: &str, to: &str) { panic!("STUB: not implemented") }

	pub fn add_method<F, I>(&mut self, name: &str, method: F)
	where
		F: Fn(&T, Params) -> I,
		I: Future<Output = Result<Value, Error>> + Send + 'static,
		F: Send + Sync + 'static,
	{ panic!("STUB: not implemented") }

	pub fn add_method_with_meta<F, I>(&mut self, name: &str, method: F)
	where
		F: Fn(&T, Params, M) -> I,
		I: Future<Output = Result<Value, Error>> + Send + 'static,
		F: Send + Sync + 'static,
	{ panic!("STUB: not implemented") }

	pub fn add_notification<F>(&mut self, name: &str, notification: F)
	where
		F: Fn(&T, Params),
		F: Send + Sync + 'static,
	{ panic!("STUB: not implemented") }

	pub fn add_notification_with_meta<F>(&mut self, name: &str, notification: F)
	where
		F: Fn(&T, Params, M),
		F: Send + Sync + 'static,
	{ panic!("STUB: not implemented") }
}

impl<T, M> crate::io::IoHandlerExtension<M> for IoDelegate<T, M>
where
	T: Send + Sync + 'static,
	M: Metadata,
{
	fn augment<S: crate::Middleware<M>>(self, handler: &mut crate::MetaIoHandler<M, S>) { panic!("STUB: not implemented") }
}

impl<T, M> IntoIterator for IoDelegate<T, M>
where
	T: Send + Sync + 'static,
	M: Metadata,
{
	type Item = (String, RemoteProcedure<M>);
	type IntoIter = std::collections::hash_map::IntoIter<String, RemoteProcedure<M>>;

	fn into_iter(self) -> Self::IntoIter { panic!("STUB: not implemented") }
}
