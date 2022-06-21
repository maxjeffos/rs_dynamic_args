use extension_lib;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct WoofInput {
    lang: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n[woof extension] start");
    let extension_root = extension_lib::extension_root()?;
    println!("[woof extension] {:?}", extension_root);

    let extension = extension_lib::extension::try_load(&extension_root)?;
    println!("[woof extension] {:#?}", extension);

    println!("reading json from stdin:");
    let std_in_str = extension_lib::read_input()?;
    println!("[woof extension] std_in_str: {:?}", std_in_str);

    let launch_codes = extension_lib::launch_codes_new::deser_launch_codes(&std_in_str)?;
    println!("[woof extension] launch_codes:\n{:#?}", launch_codes);
    println!("[woof extension] done");

    Ok(())
}
