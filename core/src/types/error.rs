
use super::Value;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorCode {
	
	ParseError,
	
	InvalidRequest,
	
	MethodNotFound,
	
	InvalidParams,
	
	InternalError,
	
	ServerError(i64),
}

impl ErrorCode {
	
	pub fn code(&self) -> i64 { panic!("STUB: not implemented") }

	pub fn description(&self) -> String { panic!("STUB: not implemented") }
}

impl From<i64> for ErrorCode {
	fn from(code: i64) -> Self { panic!("STUB: not implemented") }
}

impl<'a> Deserialize<'a> for ErrorCode {
	fn deserialize<D>(deserializer: D) -> Result<ErrorCode, D::Error>
	where
		D: Deserializer<'a>,
	{ panic!("STUB: not implemented") }
}

impl Serialize for ErrorCode {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{ panic!("STUB: not implemented") }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Error {
	
	pub code: ErrorCode,
	
	pub message: String,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Value>,
}

impl Error {
	
	pub fn new(code: ErrorCode) -> Self { panic!("STUB: not implemented") }

	pub fn parse_error() -> Self { panic!("STUB: not implemented") }

	pub fn invalid_request() -> Self { panic!("STUB: not implemented") }

	pub fn method_not_found() -> Self { panic!("STUB: not implemented") }

	pub fn invalid_params<M>(message: M) -> Self
	where
		M: Into<String>,
	{ panic!("STUB: not implemented") }

	pub fn invalid_params_with_details<M, T>(message: M, details: T) -> Error
	where
		M: Into<String>,
		T: fmt::Debug,
	{ panic!("STUB: not implemented") }

	pub fn internal_error() -> Self { panic!("STUB: not implemented") }

	pub fn invalid_version() -> Self { panic!("STUB: not implemented") }
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { panic!("STUB: not implemented") }
}

impl std::error::Error for Error {}
