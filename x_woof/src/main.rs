use extension_lib;
use extension_lib::extension_metadata;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Number, Value};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct WoofInput {
    lang: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extension_root = extension_lib::extension_root()?;
    println!("{:?}", extension_root);

    println!("extension_root: {:?}", extension_root);
    let extension_metadata_path = extension_root.join("extension.json");
    println!("extension_metadata_path: {:?}", extension_metadata_path);

    // temporarily, just reading the extension.json file from the root
    let bin_path = std::env::current_exe()?;
    let bin_dir_path = bin_path.parent().unwrap();
    let extension_metadata_path = bin_dir_path.join("extension.json");
    // let extension_metadata_path = Path::new("./extension.json");
    println!("extension_metadata_path: {:?}", extension_metadata_path);

    println!("");

    let extension_metadata =
        extension_metadata::deser_extension_metadata(&extension_metadata_path)?;
    println!("extension_metadata: {:?}", extension_metadata);

    // TODO: read this from stdin
    // let std_in_str = r#"{"debug":true,"proxy_port":64969,"args":{"lang":"en"}}"#;
    println!("reading json from stdin:");
    let std_in_str = extension_lib::read_input()?;
    println!("std_in_str: {:?}", std_in_str);

    let launch_codes = extension_lib::launch_codes_new::deser_launch_codes(&std_in_str)?;
    println!("launch_codes:\n{:#?}", launch_codes);

    Ok(())
}
