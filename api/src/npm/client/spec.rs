use std::collections::HashMap;

use serde::{de::Visitor, Deserialize};

#[derive(Deserialize, Debug)]
pub struct NpmAbbrevPackageVersionDto {
    pub name: String,
    pub version: String,
    pub license: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NpmDistTag {
    Latest,
    Next,
    Other(String),
}

struct NpmDistTagVisitor;

impl<'de> Visitor<'de> for NpmDistTagVisitor {
    type Value = NpmDistTag;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "latest" => Ok(NpmDistTag::Latest),
            "next" => Ok(NpmDistTag::Next),
            _ => Ok(NpmDistTag::Other(v.to_owned())),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v.as_str() {
            "latest" => Ok(NpmDistTag::Latest),
            "next" => Ok(NpmDistTag::Next),
            _ => Ok(NpmDistTag::Other(v)),
        }
    }
}

impl<'de> Deserialize<'de> for NpmDistTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(NpmDistTagVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct NpmAbbrevPackageDto {
    pub name: String,
    #[serde(rename(deserialize = "dist-tags"))]
    pub dist_tags: HashMap<NpmDistTag, String>,
    pub versions: HashMap<String, NpmAbbrevPackageVersionDto>,
}
