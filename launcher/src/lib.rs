use clap;
use extension_lib::extension::metadata::{self, ExtensionMetadata};

pub mod launch_codes;

pub fn make_arg_parser_config(ext_meta: &ExtensionMetadata) -> clap::App {
    let base_app = clap::Command::new("launcher")
        .about("cli arg parsing with ext passthrough")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("maxjeffos");
    // .subcommand(Command::new("version").about("show version"));

    // add extension command
    let ext_command = clap_command_from_extension_metadata_command(&ext_meta.command);
    base_app.subcommand(ext_command)
}

fn clap_command_from_extension_metadata_command(cmd: &metadata::Command) -> clap::App {
    let ext_command_description: &str = &cmd.description;
    let clap_command = clap::Command::new(&cmd.name).about(ext_command_description);

    // add options to the clap command
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
    let clap_command = clap_command.args(args);

    let mut clap_subcommands = Vec::new();
    if let Some(subcommands) = &cmd.subcommands {
        for subcommand in subcommands {
            let clap_subcommand = clap_command_from_extension_metadata_command(subcommand);
            clap_subcommands.push(clap_subcommand);
        }
    }

    clap_command.subcommands(clap_subcommands)
}
