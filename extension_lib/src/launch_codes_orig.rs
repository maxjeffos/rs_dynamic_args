use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Number, Value};
use std::fs::File;
use std::io::BufReader;
// use serde;
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtensionInput<T> {
    pub debug: bool,
    pub proxy_port: i32,
    pub args: T,
}

// aka parse_input
pub fn deser_launch_codes<'de, T>(input: &'de str) -> anyhow::Result<ExtensionInput<T>>
where
    T: Deserialize<'de>,
{
    let value = serde_json::from_str(input)?;
    Ok(value)
}

pub fn deser_launch_codes_owned<T>(input: &str) -> anyhow::Result<ExtensionInput<T>>
where
    T: DeserializeOwned,
{
    let value = serde_json::from_str(input)?;
    Ok(value)
}
