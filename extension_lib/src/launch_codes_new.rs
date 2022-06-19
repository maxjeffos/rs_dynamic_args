use std::collections::HashMap;
use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Number, Value};
use serde::de::DeserializeOwned;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtensionInput {
    pub debug: bool,
    pub proxy_port: i32,
    pub command: Command,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Command { // analogous to Command in ExtensionMetadata
    pub name: String,
    pub subcommand: Option<Box<Command>>,
    pub options: HashMap::<String, String>, // use serde_json::Value instead? but then what would we use in Go?
    pub positionals: Vec<String>,
}

// aka parse_input
pub fn deser_launch_codes<'de, T>(input: &'de str) -> anyhow::Result<ExtensionInput>
where
    T: Deserialize<'de>,
{
    let value = serde_json::from_str(input)?;
    Ok(value)
}

pub fn deser_launch_codes_owned<T>(input: &str) -> anyhow::Result<ExtensionInput>
where
    T: DeserializeOwned,
{
    let value = serde_json::from_str(input)?;
    Ok(value)
}
