use std::{borrow::Cow, fmt::Display};

use flux_nbt::{Nbt, Value};
use serde::Serialize;

use crate::err::Error;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Identifier(IdentifierInner<'static>);

impl Identifier {
    pub fn new<N: Into<String>, P: Into<String>>(namespace: N, path: P) -> Self {
        Self(IdentifierInner {
            namespace: Cow::Owned(namespace.into()),
            path: Cow::Owned(path.into()),
        })
    }

    pub fn from_string(s: &'static str) -> Result<Self, Error> {
        let split = s.split(':').collect::<Vec<&str>>();
        if split.len() != 2 {
            Err(Error::ParsingError)
        } else {
            Ok(Identifier(IdentifierInner {
                namespace: Cow::Borrowed(split[0]),
                path: Cow::Borrowed(split[1]),
            }))
        }
    }
}

impl From<Identifier> for String {
    fn from(id: Identifier) -> Self {
        format!("{}:{}", id.0.namespace, id.0.path)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.0.namespace, self.0.path))
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct IdentifierInner<'a> {
    namespace: Cow<'a, str>,
    path: Cow<'a, str>,
}

impl Nbt for Identifier {
    fn nbt(&self) -> flux_nbt::Value {
        Value::String(self.to_string())
    }
}
