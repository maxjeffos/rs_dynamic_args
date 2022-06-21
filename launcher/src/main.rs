use anyhow;
use extension_lib::extension::metadata;
use launcher::launch_codes::make_launch_codes;
use serde_json;
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{self, Stdio};

const EXTENSION_NOT_FOUND_ERROR: i32 = 100; // error code that shouldn't conflict with errors from extensions

fn get_extension_metadata_file_path() -> anyhow::Result<PathBuf> {
    let extension_root_env_var = std::env::var("EXTENSION_ROOT");

    match extension_root_env_var {
        Ok(root) => {
            let extension_root = PathBuf::from(root);
            let extension_metadata_path = extension_root.join("extension.json");
            if extension_metadata_path.exists() {
                Ok(extension_metadata_path)
            } else {
                Err(anyhow::anyhow!(
                    "extension.json not found in directory given by EXTENSION_ROOT env var"
                ))
            }
        }
        Err(e) => {
            match e {
                env::VarError::NotPresent => {
                    let current_dir =
                        std::env::current_dir().expect("failed to get current directory"); // TODO: better error handling
                    let extension_metadata_path = current_dir.join("extension.json");
                    if extension_metadata_path.exists() {
                        Ok(extension_metadata_path)
                    } else {
                        Err(anyhow::anyhow!(
                            "extension.json file not found in EXTENSION_ROOT or current directory"
                        ))
                    }
                }
                env::VarError::NotUnicode(os_string) => {
                    println!("{:?}", os_string);
                    Err(anyhow::anyhow!("failed to get EXTENSION_ROOT env var"))
                    // TODO: how can we add the e error to the anyhow error?
                    // anyhow::anyhow!(e)
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args();
    println!("{:?}", args);

    // let app_args = args.skip(1).collect::<Vec<_>>();
    // clap expects all the args, including the 0th arg, i.e. the executable
    let app_args = env::args();

    // validate the user input dynamically generated Clap config

    // because this is just the launcher, and not the full CLI, it is intented to be used with a specific extension
    // but we can't mess with the input params (because they need to simulate actual user input so the input args need to be entered exactly like they would be in the CLI), the launcher should pickup the extension 1) by default in the cwd; 2) by env var which would override the default/cwd
    // So you might do `launcher woof --lang=en` if the `woof` extension (both the extension metadata and the binary) was located in the current directory
    // Or you might do `EXTENSION_ROOT=/path/to/dir/with/extension launcher woof --lang=en`

    let extension_path = get_extension_metadata_file_path();

    match extension_path {
        Ok(extension_path) => {
            let extension_metadata = metadata::deser_extension_metadata(&extension_path)?;

            // generate the Clap / arg parsing thing dynamically based off the extension metadata
            // note: for full CLI, would need to do this based on other things, too: built-ins, all discovered extensions, and the top-level commands of the fallback CLI
            let clap_config = launcher::make_arg_parser_config(&extension_metadata);

            let matches = clap_config.get_matches_from(app_args);
            // let matches = clap_config.get_matches(); // this will pull get args from the os. The above is more portable
            println!("{:#?}", matches);

            // generate the launch codes from 1) the Clap config 2) the user input 3) the ExtensionMetadata
            let launch_codes = make_launch_codes(&matches, &extension_metadata);
            println!("\nlaunch codes: {:#?}", launch_codes);

            let launch_codes_json_string = serde_json::to_string_pretty(&launch_codes)?;
            println!(
                "\nback in main, launch codes json\n{}:",
                launch_codes_json_string
            );

            // launch the extension with the launch codes

            // TODO: I figured this out in get_extension_metadata_file_path() so need to dedupe and dekludge this
            let ext_root = extension_path.parent().unwrap();
            println!("ext_root {:?}", ext_root);

            let ext_bin_filename = format!("{}_darwin_arm64", &extension_metadata.name);
            println!("ext_bin_filename {:?}", ext_bin_filename);
            let ext_bin_path = ext_root.join(ext_bin_filename);
            println!("ext_bin_path {:?}", ext_bin_path);
            // let ext_path = std::fs::canonicalize(ext_path)?;
            // println!("\nlaunching extension at {:?}", ext_path);

            println!("\nlaunching extension at {:?}", ext_bin_path);

            let mut ext_process = process::Command::new(ext_bin_path)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .stdin(Stdio::piped())
                .env("EXTENSION_ROOT", ext_root)
                .spawn()?;

            let ext_process_stdin = ext_process.stdin.as_mut().unwrap();
            ext_process_stdin.write(launch_codes_json_string.as_bytes())?;

            let ext_process_result = ext_process.wait();
            println!("\nback from extension in launcher");
            println!("ext_process_result: {:#?}", ext_process_result);
        }
        Err(e) => {
            println!("{:?}", e);
            println!("Could not find extension path");
            std::process::exit(EXTENSION_NOT_FOUND_ERROR);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_extension_metadata_file_path {
        use super::super::*;

        use std::fs;
        use std::path::PathBuf;

        fn full_path_to_x_woof_extension_root() -> PathBuf {
            fs::canonicalize("../x_woof").unwrap()
        }

        // fn full_path_to_x_woof_extension_root() -> PathBuf {
        //     let mut current_dir = std::env::current_dir().unwrap();
        //     println!("current_dir{:?}", current_dir);

        //     current_dir.push("../x_woof");
        //     println!("current_dir{:?}", current_dir);

        //     let extension_root = current_dir.canonicalize().unwrap();
        //     println!("extension_root{:?}", extension_root);

        //     // let extension_root = PathBuf::from("../x-woof");

        //     let exists = extension_root.exists();
        //     println!("exists: {:?}", exists);

        //     let path = fs::canonicalize("../x_woof").unwrap();
        //     println!("path: {:?}", path);

        //     File::open(extension_root.join("extension.json")).unwrap();

        //     extension_root
        // }

        #[test]
        fn works_if_no_env_var_set_and_extension_metadata_file_exists_in_cwd() {
            // rather than using a test fixture which is likely to go out of data when `../x_woof/extension.json` changes, just reference it.

            let root = full_path_to_x_woof_extension_root();
            std::env::set_current_dir(root).unwrap();

            let extension_metadata_file_path = get_extension_metadata_file_path();
            assert_eq!(
                extension_metadata_file_path.unwrap(),
                fs::canonicalize("../x_woof/extension.json").unwrap()
            );
        }

        #[test]
        fn works_if_env_var_set_to_legit_directory_containing_extension_metadata_file() {
            let root = full_path_to_x_woof_extension_root();
            std::env::set_var("EXTENSION_ROOT", root);

            let extension_metadata_file_path = get_extension_metadata_file_path();
            assert_eq!(
                extension_metadata_file_path.unwrap(),
                fs::canonicalize("../x_woof/extension.json").unwrap()
            );
        }

        // if the env var is set but the file is not found, the value of the env var is invalid in any way, an Error should be returned
        #[test]
        fn returns_err_if_env_var_set_but_file_not_found() {
            // use the current directory which has no extension.json
            let current_dir = std::env::current_dir().unwrap();
            std::env::set_var("EXTENSION_ROOT", current_dir);

            let extension_metadata_file_path = get_extension_metadata_file_path();
            println!(
                "extension_metadata_file_path: {:?}",
                extension_metadata_file_path
            );
            let e = extension_metadata_file_path.unwrap_err();
            assert_eq!(
                format!("{}", e),
                "extension.json not found in directory given by EXTENSION_ROOT env var"
            );
        }

        // TODO: make tests for these other scenarios:
        // if the env var not set and an extension.json file is found in the cwd, the path to the file should be returned
        // if the env var not set and an extension.json file is not found in the cwd, an Error should be returned

        #[test]
        fn returns_none_if_no_extension_metadata_file_found_in_cwd_and_env_var_is_not_set() {

            // unimplemented!()
        }
    }

    // #[test]
    // fn generate_clap_config_for_simple_extension() {
    //     // define extension metadata by path to the x-woof file
    //     // deser the metadata
    //     // generate the Clap config dynamically
    //     // try passing it some cooked user args

    //     let extension_root = Path::from("../x-woof");
    //     let extension_metadata_path = extension_root.join("extension.json");
    //     let extension_metadata = extension_lib::deser_extension_metadata(extension_metadata_path)?;

    //     assert_eq!(0, 1);

    //     unimplemented!();
    // }

    #[test]
    fn it_works() {
        // represents `depgraph --output=json --detailed /path/to/test1 /path/to/test2`
        let app_args: Vec<String> = vec![
            "depgraph".to_string(),
            "--output=json".to_string(),
            "--detailed".to_string(),
            "/path/to/test1".to_string(),
            "/path/to/test2".to_string(),
        ];

        println!("{:?}", app_args);
    }

    // #[test]
    // fn it_works() {
    //     // represents `depgraph --output=json --detailed /path/to/test1 /path/to/test2`
    //     let app_args: Vec<String> = vec![
    //         "depgraph".to_string(),
    //         "--output=json".to_string(),
    //         "--detailed".to_string(),
    //         "/path/to/test1".to_string(),
    //         "/path/to/test2".to_string(),
    //     ];

    //     println!("{:?}", app_args);
    // }
}
