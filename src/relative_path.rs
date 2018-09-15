#![allow(dead_code)]

extern crate serde;

use std::fmt;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Error, Visitor};

#[derive(Debug, PartialEq)]
pub struct RelativePath {
    pub segments: Vec<String>
}

// IDEA: mark whether we have a path or a file!
// extra method/type or phantom type?

impl <'a> RelativePath {
    pub fn from(path: &str) -> Result<RelativePath, String> {
        ::check_and_create_segments(path).map(|s| RelativePath{segments: s})
    }
}

impl <'a> Serialize for RelativePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let path = self.segments.join("/");
        serializer.serialize_str(&path)
    }
}

struct RelativePathStringVisitor;

impl<'de> Visitor<'de> for RelativePathStringVisitor {
    type Value = RelativePath;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where E: Error {
        RelativePath::from(v).map_err(|e| Error::custom(e))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: Error {
        RelativePath::from(v).map_err(|e| Error::custom(e))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where E: Error {
        RelativePath::from(&v).map_err(|e| Error::custom(e))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for RelativePath {
    fn deserialize<D>(deserializer: D) -> Result<RelativePath, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_string(RelativePathStringVisitor)
    }
}