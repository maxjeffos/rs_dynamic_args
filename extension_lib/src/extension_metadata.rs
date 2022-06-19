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
pub struct ExtensionMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub command: Command,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub positionals: Option<Positionals>,
    pub options: Option<Vec<CommandOption>>,
    pub subcommands: Option<Vec<Command>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Positionals {
    pub name: String,
    pub cardinality: Cardinality,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Cardinality {
    #[serde(rename = "zero")]
    Zero, // may not be required

    #[serde(rename = "one")]
    One,

    #[serde(rename = "zero-or-more")]
    ZeroOrMore, // may not be required

    #[serde(rename = "one-or-more")]
    OneOrMore,

    #[serde(rename = "specific")]
    Specific(usize), // may not be required
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommandOption {
    pub name: String,
    pub shorthand: Option<char>, // should it be char?
    #[serde(rename = "type")]
    pub the_type: String,
    pub description: String,
    pub default: Option<Value>,
    // if they type is string, it would be nice to be able specify a list of possible values

    // pub allowed_values: Vec<String>, // only for string type. Ex [ "en", "de", "cz", "he" ]
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommandStringOption {
    pub name: String,
    pub shorthand: Option<char>, // should it be char?
    #[serde(rename = "type")]
    pub the_type: String,
    pub description: String,
    pub default: Option<Value>,
    // if they type is string, it would be nice to be able specify a list of possible values

    // pub allowed_values: Vec<String>, // only for string type. Ex [ "en", "de", "cz", "he" ]
}

pub fn deser_extension_metadata(path: &Path) -> anyhow::Result<ExtensionMetadata> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let extension_metadata: ExtensionMetadata = serde_json::from_reader(reader)?;
    Ok(extension_metadata)
}

pub fn deser_extension_metadata_from_reader<R>(reader: R) -> anyhow::Result<ExtensionMetadata> 
where
    R: std::io::Read
{
    let extension_metadata: ExtensionMetadata = serde_json::from_reader(reader)?;
    Ok(extension_metadata)
}
