mod field;
mod io;
mod manipulation;
mod utils;

use clap::Command;

pub fn app() -> Command {
    Command::new("rust-masm")
        .about("A CLI for Rust MASM")
        .version("0.1.0")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .author("KorieDrakeChaney")
        .subcommands(io::commands())
        .subcommands(field::commands())
        .subcommands(manipulation::commands())
        .subcommands(utils::commands())
}
