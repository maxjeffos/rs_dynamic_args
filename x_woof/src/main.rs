use extension_lib;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use serde_json::{Error, Number, Value};
use serde_derive::{Deserialize, Serialize};
use extension_lib::extension_metadata;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct WoofInput {
    lang: String,
}

// fn temp_get_extension_json() -> String {
//     // let mut file = File::open("extension.json").unwrap();
//     // let mut contents = String::new();
//     let contents = fs::read_to_string("./extension.json").expect("file not found");
//     contents
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extension_root = extension_lib::extension_root()?;
    println!("{:?}", extension_root);

    let extension_metadata_path = extension_root.join("extension.json");
    println!("extension_metadata_path: {:?}", extension_metadata_path);

    // temporarily, just reading the extension.json file from the root
    let extension_metadata_path = Path::new("./extension.json");
    println!("extension_metadata_path: {:?}", extension_metadata_path);

    println!("");

    let extension_metadata = extension_metadata::deser_extension_metadata(&extension_metadata_path)?;
    println!("extension_metadata: {:?}", extension_metadata);

    // TODO: read this from stdin
    let std_in_str = r#"{"debug":true,"proxy_port":64969,"args":{"lang":"en"}}"#;
    println!("std_in_str: {:?}", std_in_str);

    // let v: Value = serde_json::from_str(std_in_str).unwrap();
    // println!("v: {:?}", v);

    // let woof_input = serde_json::from_str::<WoofInput>(std_in_str).unwrap();

    let woof_input = extension_lib::launch_codes_orig::deser_launch_codes::<WoofInput>(std_in_str)?;
    println!("\nwoof_input: {:?}", woof_input);
    
    let woof_input_owned = extension_lib::launch_codes_orig::deser_launch_codes::<WoofInput>(std_in_str)?;
    println!("\nwoof_input_owned: {:?}", woof_input);

    // Now we have a strongly typed struct specific to this extension.
    // Could have tooling that ensures that generates the correct struct from the
    // extension.json file.

    println!("woof in {}", woof_input.args.lang);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_trivial() {
        assert_eq!(1, 1);
    }
}
