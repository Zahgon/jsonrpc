
use super::{Error, ErrorCode, Id, Value, Version};
use crate::Result as CoreResult;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Success {
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jsonrpc: Option<Version>,
	
	pub result: Value,
	
	pub id: Id,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Failure {
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jsonrpc: Option<Version>,
	
	pub error: Error,
	
	pub id: Id,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Output {
	
	Success(Success),
	
	Failure(Failure),
}

impl Output {
	
	pub fn from(result: CoreResult<Value>, id: Id, jsonrpc: Option<Version>) -> Self { panic!("STUB: not implemented") }

	pub fn invalid_request(id: Id, jsonrpc: Option<Version>) -> Self { panic!("STUB: not implemented") }

	pub fn version(&self) -> Option<Version> { panic!("STUB: not implemented") }

	pub fn id(&self) -> &Id { panic!("STUB: not implemented") }
}

impl From<Output> for CoreResult<Value> {
	
	fn from(output: Output) -> CoreResult<Value> { panic!("STUB: not implemented") }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Response {
	
	Single(Output),
	
	Batch(Vec<Output>),
}

impl Response {
	
	pub fn from(error: Error, jsonrpc: Option<Version>) -> Self { panic!("STUB: not implemented") }

	pub fn from_json(s: &str) -> Result<Self, serde_json::Error> { panic!("STUB: not implemented") }
}

impl From<Failure> for Response {
	fn from(failure: Failure) -> Self { panic!("STUB: not implemented") }
}

impl From<Success> for Response {
	fn from(success: Success) -> Self { panic!("STUB: not implemented") }
}

#[test]
fn success_output_serialize() {
	use serde_json;
	use serde_json::Value;

	let so = Output::Success(Success {
		jsonrpc: Some(Version::V2),
		result: Value::from(1),
		id: Id::Num(1),
	});

	let serialized = serde_json::to_string(&so).unwrap();
	assert_eq!(serialized, r#"{"jsonrpc":"2.0","result":1,"id":1}"#);
}

#[test]
fn success_output_deserialize() {
	use serde_json;
	use serde_json::Value;

	let dso = r#"{"jsonrpc":"2.0","result":1,"id":1}"#;

	let deserialized: Output = serde_json::from_str(dso).unwrap();
	assert_eq!(
		deserialized,
		Output::Success(Success {
			jsonrpc: Some(Version::V2),
			result: Value::from(1),
			id: Id::Num(1)
		})
	);
}

#[test]
fn failure_output_serialize() {
	use serde_json;

	let fo = Output::Failure(Failure {
		jsonrpc: Some(Version::V2),
		error: Error::parse_error(),
		id: Id::Num(1),
	});

	let serialized = serde_json::to_string(&fo).unwrap();
	assert_eq!(
		serialized,
		r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}"#
	);
}

#[test]
fn failure_output_serialize_jsonrpc_1() {
	use serde_json;

	let fo = Output::Failure(Failure {
		jsonrpc: None,
		error: Error::parse_error(),
		id: Id::Num(1),
	});

	let serialized = serde_json::to_string(&fo).unwrap();
	assert_eq!(
		serialized,
		r#"{"error":{"code":-32700,"message":"Parse error"},"id":1}"#
	);
}

#[test]
fn failure_output_deserialize() {
	use serde_json;

	let dfo = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}"#;

	let deserialized: Output = serde_json::from_str(dfo).unwrap();
	assert_eq!(
		deserialized,
		Output::Failure(Failure {
			jsonrpc: Some(Version::V2),
			error: Error::parse_error(),
			id: Id::Num(1)
		})
	);
}

#[test]
fn single_response_deserialize() {
	use serde_json;
	use serde_json::Value;

	let dsr = r#"{"jsonrpc":"2.0","result":1,"id":1}"#;

	let deserialized: Response = serde_json::from_str(dsr).unwrap();
	assert_eq!(
		deserialized,
		Response::Single(Output::Success(Success {
			jsonrpc: Some(Version::V2),
			result: Value::from(1),
			id: Id::Num(1)
		}))
	);
}

#[test]
fn batch_response_deserialize() {
	use serde_json;
	use serde_json::Value;

	let dbr = r#"[{"jsonrpc":"2.0","result":1,"id":1},{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}]"#;

	let deserialized: Response = serde_json::from_str(dbr).unwrap();
	assert_eq!(
		deserialized,
		Response::Batch(vec![
			Output::Success(Success {
				jsonrpc: Some(Version::V2),
				result: Value::from(1),
				id: Id::Num(1)
			}),
			Output::Failure(Failure {
				jsonrpc: Some(Version::V2),
				error: Error::parse_error(),
				id: Id::Num(1)
			})
		])
	);
}

#[test]
fn handle_incorrect_responses() {
	use serde_json;

	let dsr = r#"
{
	"id": 2,
	"jsonrpc": "2.0",
	"result": "0x62d3776be72cc7fa62cad6fe8ed873d9bc7ca2ee576e400d987419a3f21079d5",
	"error": {
		"message": "VM Exception while processing transaction: revert",
		"code": -32000,
		"data": {}
	}
}"#;

	let deserialized: Result<Response, _> = serde_json::from_str(dsr);
	assert!(
		deserialized.is_err(),
		"Expected error when deserializing invalid payload."
	);
}

#[test]
fn should_parse_empty_response_as_batch() {
	use serde_json;

	let dsr = r#""#;

	let deserialized1: Result<Response, _> = serde_json::from_str(dsr);
	let deserialized2: Result<Response, _> = Response::from_json(dsr);
	assert!(
		deserialized1.is_err(),
		"Empty string is not valid JSON, so we should get an error."
	);
	assert_eq!(deserialized2.unwrap(), Response::Batch(vec![]));
}
