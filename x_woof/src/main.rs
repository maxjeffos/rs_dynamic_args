use extension_lib;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct WoofInput {
    lang: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extension_root = extension_lib::extension_root()?;
    println!("{:?}", extension_root);

    let extension = extension_lib::extension::try_load(&extension_root)?;
    println!("{:#?}", extension);

    println!("reading json from stdin:");
    let std_in_str = extension_lib::read_input()?;
    println!("std_in_str: {:?}", std_in_str);

    let launch_codes = extension_lib::launch_codes_new::deser_launch_codes(&std_in_str)?;
    println!("launch_codes:\n{:#?}", launch_codes);

    Ok(())
}
