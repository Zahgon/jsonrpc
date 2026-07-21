
use super::{Id, Params, Version};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MethodCall {
	
	pub jsonrpc: Option<Version>,
	
	pub method: String,
	
	#[serde(default = "default_params")]
	pub params: Params,
	
	pub id: Id,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Notification {
	
	pub jsonrpc: Option<Version>,
	
	pub method: String,
	
	#[serde(default = "default_params")]
	pub params: Params,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Call {
	
	MethodCall(MethodCall),
	
	Notification(Notification),
	
	Invalid {
		
		#[serde(default = "default_id")]
		id: Id,
	},
}

fn default_params() -> Params { panic!("STUB: not implemented") }

fn default_id() -> Id { panic!("STUB: not implemented") }

impl From<MethodCall> for Call {
	fn from(mc: MethodCall) -> Self { panic!("STUB: not implemented") }
}

impl From<Notification> for Call {
	fn from(n: Notification) -> Self { panic!("STUB: not implemented") }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Request {
	
	Single(Call),
	
	Batch(Vec<Call>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::Value;

	#[test]
	fn method_call_serialize() {
		use serde_json;
		use serde_json::Value;

		let m = MethodCall {
			jsonrpc: Some(Version::V2),
			method: "update".to_owned(),
			params: Params::Array(vec![Value::from(1), Value::from(2)]),
			id: Id::Num(1),
		};

		let serialized = serde_json::to_string(&m).unwrap();
		assert_eq!(
			serialized,
			r#"{"jsonrpc":"2.0","method":"update","params":[1,2],"id":1}"#
		);
	}

	#[test]
	fn notification_serialize() {
		use serde_json;
		use serde_json::Value;

		let n = Notification {
			jsonrpc: Some(Version::V2),
			method: "update".to_owned(),
			params: Params::Array(vec![Value::from(1), Value::from(2)]),
		};

		let serialized = serde_json::to_string(&n).unwrap();
		assert_eq!(serialized, r#"{"jsonrpc":"2.0","method":"update","params":[1,2]}"#);
	}

	#[test]
	fn call_serialize() {
		use serde_json;
		use serde_json::Value;

		let n = Call::Notification(Notification {
			jsonrpc: Some(Version::V2),
			method: "update".to_owned(),
			params: Params::Array(vec![Value::from(1)]),
		});

		let serialized = serde_json::to_string(&n).unwrap();
		assert_eq!(serialized, r#"{"jsonrpc":"2.0","method":"update","params":[1]}"#);
	}

	#[test]
	fn request_serialize_batch() {
		use serde_json;

		let batch = Request::Batch(vec![
			Call::MethodCall(MethodCall {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![Value::from(1), Value::from(2)]),
				id: Id::Num(1),
			}),
			Call::Notification(Notification {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![Value::from(1)]),
			}),
		]);

		let serialized = serde_json::to_string(&batch).unwrap();
		assert_eq!(
			serialized,
			r#"[{"jsonrpc":"2.0","method":"update","params":[1,2],"id":1},{"jsonrpc":"2.0","method":"update","params":[1]}]"#
		);
	}

	#[test]
	fn notification_deserialize() {
		use serde_json;
		use serde_json::Value;

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": [1,2]}"#;
		let deserialized: Notification = serde_json::from_str(s).unwrap();

		assert_eq!(
			deserialized,
			Notification {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![Value::from(1), Value::from(2)])
			}
		);

		let s = r#"{"jsonrpc": "2.0", "method": "foobar"}"#;
		let deserialized: Notification = serde_json::from_str(s).unwrap();

		assert_eq!(
			deserialized,
			Notification {
				jsonrpc: Some(Version::V2),
				method: "foobar".to_owned(),
				params: Params::None,
			}
		);

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": [1,2], "id": 1}"#;
		let deserialized: Result<Notification, _> = serde_json::from_str(s);
		assert!(deserialized.is_err());
	}

	#[test]
	fn call_deserialize() {
		use serde_json;

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": [1]}"#;
		let deserialized: Call = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Call::Notification(Notification {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![Value::from(1)])
			})
		);

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": [1], "id": 1}"#;
		let deserialized: Call = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Call::MethodCall(MethodCall {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![Value::from(1)]),
				id: Id::Num(1)
			})
		);

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": [], "id": 1}"#;
		let deserialized: Call = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Call::MethodCall(MethodCall {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![]),
				id: Id::Num(1)
			})
		);

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": null, "id": 1}"#;
		let deserialized: Call = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Call::MethodCall(MethodCall {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::None,
				id: Id::Num(1)
			})
		);

		let s = r#"{"jsonrpc": "2.0", "method": "update", "id": 1}"#;
		let deserialized: Call = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Call::MethodCall(MethodCall {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::None,
				id: Id::Num(1)
			})
		);
	}

	#[test]
	fn request_deserialize_batch() {
		use serde_json;

		let s = r#"[{}, {"jsonrpc": "2.0", "method": "update", "params": [1,2], "id": 1},{"jsonrpc": "2.0", "method": "update", "params": [1]}]"#;
		let deserialized: Request = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Request::Batch(vec![
				Call::Invalid { id: Id::Null },
				Call::MethodCall(MethodCall {
					jsonrpc: Some(Version::V2),
					method: "update".to_owned(),
					params: Params::Array(vec![Value::from(1), Value::from(2)]),
					id: Id::Num(1)
				}),
				Call::Notification(Notification {
					jsonrpc: Some(Version::V2),
					method: "update".to_owned(),
					params: Params::Array(vec![Value::from(1)])
				})
			])
		)
	}

	#[test]
	fn request_invalid_returns_id() {
		use serde_json;

		let s = r#"{"id":120,"method":"my_method","params":["foo", "bar"],"extra_field":[]}"#;
		let deserialized: Request = serde_json::from_str(s).unwrap();

		match deserialized {
			Request::Single(Call::Invalid { id: Id::Num(120) }) => {}
			_ => panic!("Request wrongly deserialized: {:?}", deserialized),
		}
	}
}
