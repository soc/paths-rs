#![allow(dead_code)]

use std::fmt;
use std::path::PathBuf;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Error, Visitor};
use dirs;
use relative_path::RelativePath;

#[derive(Debug, PartialEq)]
pub struct AbsolutePath {
    segments: Vec<PathSegment>
}

#[derive(Clone, Debug, PartialEq)]
enum PathSegment {
    Literal(String),
    Placeholder(String)
}

impl PathSegment {
    fn to_pathbuf(self) -> Option<PathBuf> {
        match &self {
            PathSegment::Literal(value) => Some(PathBuf::from(value)),
            ps if ps == &root_dir()     => unimplemented!(),
            ps if ps == &home_dir()     => dirs::home_dir(),
            ps if ps == &current_dir()  => unimplemented!(),
            ps if ps == &cache_dir()    => dirs::cache_dir(),
            ps if ps == &config_dir()   => dirs::config_dir(),
            ps if ps == &data_dir()     => dirs::data_dir(),
            PathSegment::Placeholder(_) => unimplemented!()
        }
    }
}

const ROOT_DIR:    &'static str = "/";
const HOME_DIR:    &'static str = "<HOME_DIR>";
const CURRENT_DIR: &'static str = "<CURRENT_DIR>";
const CACHE_DIR:   &'static str = "<CACHE_DIR>";
const CONFIG_DIR:  &'static str = "<CONFIG_DIR>";
const DATA_DIR:    &'static str = "<DATA_DIR>";

fn root_dir() -> PathSegment {
    PathSegment::Placeholder(ROOT_DIR.to_string())
}

fn home_dir() -> PathSegment {
    PathSegment::Placeholder(HOME_DIR.to_string())
}

fn current_dir() -> PathSegment {
    PathSegment::Placeholder(CURRENT_DIR.to_string())
}

fn cache_dir() -> PathSegment {
    PathSegment::Placeholder(CACHE_DIR.to_string())
}

fn config_dir() -> PathSegment {
    PathSegment::Placeholder(CONFIG_DIR.to_string())
}

fn data_dir() -> PathSegment {
    PathSegment::Placeholder(DATA_DIR.to_string())
}

impl AbsolutePath {
    pub fn root_dir() -> AbsolutePath {
        AbsolutePath { segments: vec![root_dir()] }
    }

    pub fn home_dir() -> AbsolutePath {
        AbsolutePath { segments: vec![home_dir()] }
    }

    pub fn current_dir() -> AbsolutePath {
        AbsolutePath { segments: vec![current_dir()] }
    }

    pub fn cache_dir() -> AbsolutePath {
        AbsolutePath { segments: vec![cache_dir()] }
    }

    pub fn config_dir() -> AbsolutePath {
        AbsolutePath { segments: vec![config_dir()] }
    }

    pub fn data_dir() -> AbsolutePath {
        AbsolutePath { segments: vec![data_dir()] }
    }

    pub fn from(path: &str) -> Result<AbsolutePath, String> {
        ::check_and_create_segments(path)
          .map(|s| s.iter().map(|t| PathSegment::Literal(t.to_string())).collect())
          .map(|l| AbsolutePath{segments: l})
    }

    /*
    pub from(project_dir: ProjectDir) -> Result<AbsolutePath, String> {
        unimplemented!();
    }
    */

    pub fn append(&mut self, mut path: RelativePath) {
        let mut lits = path.segments.iter_mut().map(|s| PathSegment::Literal(s.to_string())).collect();
        self.segments.append(&mut lits);
    }

    pub fn extend(&mut self, path: RelativePath) {
        let lits = path.segments.iter().map(|s| PathSegment::Literal(s.to_string()));
        self.segments.extend(lits);
    }

    pub fn replace(mut self, path: AbsolutePath) -> AbsolutePath {
        self.segments = path.segments.clone();
        self
    }

    pub fn to_pathbuf(self) -> Option<PathBuf> {
        self.segments.into_iter().map(|s| s.to_pathbuf()).collect()
    }
}

impl <'a> Serialize for AbsolutePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut root_path;

        if self.segments.is_empty() {
            root_path = String::from("/");
        } else {
            root_path = String::from("");
            let mut segs = self.segments.iter();
            while let Some(seg) = segs.next() {
                let str = match seg {
                    PathSegment::Placeholder(str) => str,
                    PathSegment::Literal(str)     => str
                };
                root_path.push_str("/");
                root_path.push_str(str);
            }
        }
        serializer.serialize_str(&root_path)
    }
}

struct AbsolutePathStringVisitor;

impl<'de> Visitor<'de> for AbsolutePathStringVisitor {
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

impl<'de: 'a, 'a> Deserialize<'de> for AbsolutePath {
    fn deserialize<D>(deserializer: D) -> Result<AbsolutePath, D::Error>
        where D: Deserializer<'de> {
        deserializer.deserialize_string(AbsolutePathStringVisitor)
    }
}
