use std::collections::HashMap;

use clap;
use clap::ArgMatches;
use extension_lib::extension::metadata;
use extension_lib::launch_codes_new;

// Take in the app args from the user CLI input and the ExtensionMetadata and generate the launch codes
pub fn make_launch_codes(
    // app_args: &[String],
    matches: &clap::ArgMatches,
    ext_meta: &metadata::ExtensionMetadata,
) -> launch_codes_new::ExtensionInput {
    println!("in make_launch_codes");
    println!("matches: {:#?}", matches);

    let (cmd_name, cmd_matches) = matches
        .subcommand()
        .expect("there should at least be a top-level command (for the extension)");
    println!("subcommand: {:?}", cmd_name);

    let options = get_options_from_subcommand(&ext_meta.command, cmd_matches);

    println!("options: {:?}", options);

    let mut command = launch_codes_new::Command {
        name: cmd_name.to_string(),
        subcommand: None,
        options,
        positionals: Vec::new(),
    };

    // recursively add all the subcommands
    add_subcommands(&mut command, cmd_matches, &ext_meta.command);
    println!("\nLaunch Codes Command:\n{:#?}", command);

    let launch_codes = launch_codes_new::ExtensionInput {
        debug: true,
        proxy_port: 12345,
        command,
    };

    launch_codes
}

/// Recursively add all the subcommands (and their options) to the launch_codes_command
fn add_subcommands(
    launch_codes_command: &mut launch_codes_new::Command,
    matches: &clap::ArgMatches,
    extension_metadata_command: &metadata::Command,
) {
    println!("in add_subcommands");
    println!("  - lanch_codes_command: {:?}", launch_codes_command);
    println!("  - matches: {:?}", matches);

    if let Some((subcommand_name, subcommand_matches)) = matches.subcommand() {
        println!("  - subcommand_name: {:?}", subcommand_name);
        println!("  - subcommand_matches: {:#?}", subcommand_matches);

        // add the options for the subcommand
        // start by getting the appropriate subcommand from the extension_metadata_command

        let next_extension_metadata_command = extension_metadata_command
            .subcommands
            .get(subcommand_name)
            .expect("there should be a subcommand with the name of the ArgMatches subcommand"); // TODO: return error from this fn rather than panic

        let options =
            get_options_from_subcommand(next_extension_metadata_command, subcommand_matches);
        println!("  - options: {:?}", options);

        let launch_codes_subcommand = launch_codes_new::Command {
            name: subcommand_name.to_string(),
            subcommand: None,
            options,
            positionals: Vec::new(),
        };
        launch_codes_command.subcommand = Some(Box::new(launch_codes_subcommand));

        // recursively add the rest of the subcommands and their options
        add_subcommands(
            launch_codes_command,
            subcommand_matches,
            next_extension_metadata_command,
        );
    }
}

fn get_options_from_subcommand(
    ext_meta_command: &metadata::Command,
    clap_command_matches: &ArgMatches,
) -> HashMap<String, String> {
    let mut options = HashMap::new();
    if let Some(ext_metadata_command_options) = &ext_meta_command.options {
        for o in ext_metadata_command_options {
            let name = o.name.to_string();

            let maybe_value = clap_command_matches.value_of(&name);
            if let Some(value) = maybe_value {
                options.insert(name.to_string(), value.to_string());
            } else {
                // use the default value which must (by convention) exist since we know that required is false in this case and we have a rule that says
                // if required is false, there must be a default value
                println!("adding default value for option: {:?}", &name);
                let default_value = o
                    .default
                    .as_ref()
                    .expect("if required is false, there must be a default value"); // TODO: enforce this with the type system

                println!("default value: {:?}", default_value);
                let default_value_string = default_value.to_string();
                println!("default_value_string: {:?}", default_value_string);

                // default_value is a serde_json::Value type - need to convert it to a String (for now , at least, and maybe ultimately into bool, etc)
                if default_value.is_string() {
                    println!("option default value is a string");
                    let default_value_string = default_value
                        .as_str()
                        .expect("default value must be convertible to a string")
                        .to_owned();
                    println!("default_value_string: {:?}", default_value_string);
                    options.insert(name.to_string(), default_value_string);
                } else if default_value.is_boolean() {
                    println!("option default value is a boolean");
                    let v = default_value
                        .as_bool()
                        .expect("default value must be convertible to a boolean")
                        .to_string();
                    println!("v: {:?}", v);
                    options.insert(name.to_string(), v);
                }
            }
        }
    }

    options
}
