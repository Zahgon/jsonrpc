
#![deny(missing_docs)]

use std::pin::Pin;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "futures")]
pub use futures;
#[cfg(feature = "futures-executor")]
pub use futures_executor;
pub use futures_util;

pub extern crate serde;

pub extern crate serde_json;

mod calls;
mod io;

pub mod delegates;
pub mod middleware;
pub mod types;

pub type Result<T> = std::result::Result<T, Error>;

pub type BoxFuture<T> = Pin<Box<dyn std::future::Future<Output = T> + Send>>;

pub use crate::calls::{
	Metadata, RemoteProcedure, RpcMethod, RpcMethodSimple, RpcMethodSync, RpcNotification, RpcNotificationSimple,
	WrapFuture,
};
pub use crate::delegates::IoDelegate;
pub use crate::io::{
	Compatibility, FutureOutput, FutureResponse, FutureResult, FutureRpcResult, IoHandler, IoHandlerExtension,
	MetaIoHandler,
};
pub use crate::middleware::{Middleware, Noop as NoopMiddleware};
pub use crate::types::*;

use serde_json::Error as SerdeError;

pub fn serde_from_str<'a, T>(input: &'a str) -> std::result::Result<T, SerdeError>
where
	T: serde::de::Deserialize<'a>,
{ panic!("STUB: not implemented") }
