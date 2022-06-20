use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Number, Value};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtensionInput {
    pub debug: bool,
    pub proxy_port: i32,
    pub command: Command,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Command {
    // analogous to Command in ExtensionMetadata
    pub name: String,
    pub subcommand: Option<Box<Command>>,
    pub options: HashMap<String, String>, // use serde_json::Value instead? but then what would we use in Go?
    pub positionals: Vec<String>,
}

// aka parse_input
pub fn deser_launch_codes(input: &str) -> Result<ExtensionInput, serde_json::Error> {
    serde_json::from_str(input)
}
