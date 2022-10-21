use extension_lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n[dg extension] start");
    let extension_root = extension_lib::extension_root()?;
    println!("[dg extension] extension_root{:?}", extension_root);

    let extension = extension_lib::extension::try_load(&extension_root)?;
    println!("[dg extension] {:#?}", extension);

    let std_in_str = extension_lib::read_input()?;
    println!("[dg extension] std_in_str: {:?}", std_in_str);

    let launch_codes = extension_lib::launch_codes_new::deser_launch_codes(&std_in_str)?;
    println!("[dg extension] launch_codes:\n{:#?}", launch_codes);
    println!("[dg extension] done");

    // extension goes here

    Ok(())
}
