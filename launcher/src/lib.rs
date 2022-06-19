use anyhow;
use extension_lib::extension_metadata::{self, ExtensionMetadata};
use std::env;
use std::path::{Path, PathBuf};
// use clap::{App, Arg, Command, Subcommand};
use clap;

pub mod launch_codes;

pub fn make_arg_parser_config(ext_meta: &ExtensionMetadata) -> clap::App {
    let base_app = clap::Command::new("launcher")
        .about("cli arg parsing with ext passthrough")
        .version("5.2.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("maxjeffos");
    // .subcommand(Command::new("version").about("show version"));

    // add extension command

    let x_command = clap_command_from_extension_metadata_command(&ext_meta.command);
    let the_app = base_app.subcommand(x_command);

    the_app
}

fn clap_command_from_extension_metadata_command(cmd: &extension_metadata::Command) -> clap::App {
    let ext_command_description: &str = &cmd.description;
    let extension_command_base = clap::Command::new(&cmd.name).about(ext_command_description);

    // add options to the extension command
    let mut args = Vec::new();
    if let Some(options) = &cmd.options {
        for opt in options {
            let name: &str = &opt.name;

            let maybe_shorthand = opt.shorthand;
            println!("shorthand: {:?}", maybe_shorthand);

            let description: &str = &opt.description;

            let arg = clap::Arg::new(name)
                .long(name)
                .help(description)
                .takes_value(true)
                .required(opt.required);

            let arg = if let Some(shorthand) = maybe_shorthand {
                arg.short(shorthand)
            } else {
                arg
            };

            args.push(arg);
        }
    }
    let extension_command_base = extension_command_base.args(args);

    let mut clap_subcommands = Vec::new();
    if let Some(subcommands) = &cmd.subcommands {
        for subcommand in subcommands {
            let clap_subcommand = clap_command_from_extension_metadata_command(subcommand);
            clap_subcommands.push(clap_subcommand);
        }
    }
    println!("clap_subcommands.len(): {}", clap_subcommands.len());
    let extension_command_base = extension_command_base.subcommands(clap_subcommands);

    extension_command_base
}
