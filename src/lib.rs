#![allow(dead_code)]

extern crate serde;

use std::fmt;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Error, Visitor};

#[derive(Debug, PartialEq)]
struct AbsolutePath {
    segments: Vec<String>
}

#[derive(Debug)]
struct RelativePath {
    segments: Vec<String>
}

// IDEA: mark whether we have a path or a file!
// extra method/type or phantom type?

impl AbsolutePath {
    pub fn root_dir() -> AbsolutePath {
        AbsolutePath { segments: vec!["/".into()] }
    }

    pub fn current_dir() -> AbsolutePath {
        AbsolutePath { segments: vec!["$PATH_CURRENT".into()] }
    }

    pub fn from(path: &str) -> Result<AbsolutePath, String> {
        check_and_create_segments(path).map(|s| AbsolutePath{segments: s})
    }

    /*
    pub from(project_dir: ProjectDir) -> Result<AbsolutePath, String> {
        unimplemented!();
    }
    */

    pub fn append(&mut self, mut path: RelativePath) {
        self.segments.append(&mut path.segments);
    }

    pub fn extend(&mut self, path: RelativePath) {
        self.segments.extend(path.segments);
    }

    pub fn replace(mut self, path: AbsolutePath) -> AbsolutePath {
        self.segments = path.segments.clone();
        self
    }
}

impl <'a> RelativePath {
    pub fn from(path: &str) -> Result<RelativePath, String> {
        check_and_create_segments(path).map(|s| RelativePath{segments: s})
    }
}

impl <'a> Serialize for AbsolutePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let path = self.segments.join("/");
        let mut root_path = String::from("/");
        root_path.push_str(&path);
        serializer.serialize_str(&root_path)
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = AbsolutePath;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where E: Error {
        AbsolutePath::from(v).map_err(|e| Error::custom(e))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: Error {
        AbsolutePath::from(v).map_err(|e| Error::custom(e))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where E: Error {
        AbsolutePath::from(&v).map_err(|e| Error::custom(e))
    }
}

"/$CONFIG_DIR/myapp/..."

impl<'de: 'a, 'a> Deserialize<'de> for AbsolutePath {
    fn deserialize<D>(deserializer: D) -> Result<AbsolutePath, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_string(StringVisitor)
    }
}

fn check_and_create_segments(path: &str) -> Result<Vec<String>, String> {
    let invalid = path.find(is_invalid_char);
    if invalid.is_some() {
        let index = invalid.unwrap();
        Err(format!("Invalid char {} at position {}", path.chars().nth(index).unwrap(), index))?;
    }
    Ok(path.split("/").filter(|&s| s != "").map(String::from).collect())
}

fn is_invalid_char(c: char) -> bool {
    match c {
        '\\' | '?' | '%' | '*' | ':' | '|' | '"' | '<' | '>' => true,
        _ => false
    }
}

#[cfg(test)]
mod test;
