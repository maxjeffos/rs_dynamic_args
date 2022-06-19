use std::collections::HashMap;

use clap;
use extension_lib::extension_metadata;
use extension_lib::launch_codes_new;

// Take in the app args from the user CLI input and the ExtensionMetadata and generate the launch codes
// TODO: should the launch codes returned here be an object or a String?
pub fn make_launch_codes(
    // app_args: &[String],
    matches: &clap::ArgMatches,
    ext_meta: &extension_metadata::ExtensionMetadata,
) -> launch_codes_new::ExtensionInput {
    println!("in make_launch_codes");
    // println!("app_args: {:?}", app_args);
    println!("matches: {:#?}", matches);

    // make an return an ExtensionInput object based on matches
    // ideally, we could use a standard algorithm that'll work for anything

    let v = matches;
    println!("matches: {:?}", v);

    let (cmd_name, cmd_matches) = matches
        .subcommand()
        .expect("there should at least be a top-level command (for the extension)");
    println!("subcommand: {:?}", cmd_name);

    // add the options for the top-level command
    // but they're in the .args field of matches which is private
    // So we need to iterate over the .options of the extension_metadata::Command and, for each one,
    // use cmd_matches.value_of to get the value of the option from the user CLI input
    // let extension_metadata_command = &ext_meta.command;
    let mut options = HashMap::new();
    if let Some(ext_metadata_command_options) = &ext_meta.command.options {
        for o in ext_metadata_command_options {
            let name = o.name.to_string();
            let value = cmd_matches
                .value_of(&name)
                .expect("there should be a value for the option");
            options.insert(name.to_string(), value.to_string());
        }
    }

    println!("options: {:?}", options);

    let mut command = launch_codes_new::Command {
        name: cmd_name.to_string(),
        subcommand: None,
        options,
        positionals: Vec::new(),
    };

    // recursively add all the subcommands
    mutable_co_spelunk(&mut command, cmd_matches, &ext_meta.command);

    println!("\nLaunch Codes Command:\n{:#?}", command);

    // matches.value_of("debug").unwrap_or("false");

    let launch_codes = launch_codes_new::ExtensionInput {
        debug: true,
        proxy_port: 12345,
        command,
    };

    // problems
    // this is the command I need to use: `EXTENSION_ROOT=/Users/jeff/repos/maxjeffos/rs_dynamic_args/x_dg cr -- depgraph --verbose=false test --detailed=true --output=json`
    // all of the options are mandatory. this is relatively easy to fix.
    // the --verbose option is required at the top level which is kinda weird. I'd think, even if it was mandatory, it would only be valid if no subcommands were used.
    // not adding any data for any of the args/options.

    // the woof command is:
    // `EXTENSION_ROOT=/Users/jeff/repos/maxjeffos/rs_dynamic_args/x_woof cr -- woof --lang=fr`

    launch_codes
}

// initially create a launch_codes_new::Command with the 0th subcommand (if the 0th subcommand dne, it's an error).
// then, recursively spelunk down the tree of subcommands with a fn that takes a mutable reference to the launch_codes_new::Command

fn mutable_co_spelunk(
    lanch_codes_command: &mut launch_codes_new::Command,
    matches: &clap::ArgMatches,
    extension_metadata_command: &extension_metadata::Command,
) {
    println!("in mutable_co_spelunk");
    println!("  - lanch_codes_command: {:?}", lanch_codes_command);
    println!("  - matches: {:?}", matches);

    // let mut subcommands = matches.subcommand();
    // println!("subcommands: {:?}", subcommands);

    if let Some(x) = matches.subcommand() {
        println!("x: {:?}", x);
        let (x_cmd_name, x_cmd_matches) = x;

        println!("  - x_cmd_name: {:?}", x_cmd_name);
        println!("  - x_cmd_matches: {:#?}", x_cmd_matches);

        // add the options for the subcommand
        // start by getting the appropriate subcommand from the extension_metadata_command

        let next_extension_metadata_commands = extension_metadata_command.subcommands.as_ref().expect("if there is an ArgMatches subcommand here, there must be an extension metadata subcommand");
        let next_extension_metadata_command = next_extension_metadata_commands
            .iter()
            .find(|x| x.name == x_cmd_name)
            .expect("there should be a subcommand with the name of the ArgMatches subcommand");
        println!(
            "  - next_extension_metadata_command.name: {:?}",
            next_extension_metadata_command.name
        );

        // get the options for the subcommand
        let mut options = HashMap::new();
        if let Some(ext_metadata_command_options) = &next_extension_metadata_command.options {
            for o in ext_metadata_command_options {
                let name = o.name.to_string();
                let value = x_cmd_matches
                    .value_of(&name)
                    .expect("there should be a value for the option");
                options.insert(name.to_string(), value.to_string());
            }
        }

        println!("  - options: {:?}", options);

        // create new the command
        let x_command = launch_codes_new::Command {
            name: x_cmd_name.to_string(),
            subcommand: None,
            options,
            positionals: Vec::new(),
        };
        lanch_codes_command.subcommand = Some(Box::new(x_command));

        // recursively add the rest of the subcommands and their options
        mutable_co_spelunk(
            lanch_codes_command,
            x_cmd_matches,
            next_extension_metadata_command,
        );
    }
}
