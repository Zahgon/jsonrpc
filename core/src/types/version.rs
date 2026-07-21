
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Version {
	
	V2,
}

impl Serialize for Version {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{ panic!("STUB: not implemented") }
}

impl<'a> Deserialize<'a> for Version {
	fn deserialize<D>(deserializer: D) -> Result<Version, D::Error>
	where
		D: Deserializer<'a>,
	{ panic!("STUB: not implemented") }
}

struct VersionVisitor;

impl<'a> Visitor<'a> for VersionVisitor {
	type Value = Version;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result { panic!("STUB: not implemented") }

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{ panic!("STUB: not implemented") }
}
