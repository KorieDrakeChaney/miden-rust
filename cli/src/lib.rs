mod field;
mod io;
mod manipulation;
mod utils;

pub use field::HELP as FIELD_HELP;
pub use io::HELP as IO_HELP;
pub use manipulation::HELP as MANIPULATION_HELP;

use clap::Command;

pub const APP_HELP: &'static str = "
USAGE:
    rust-masm [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    --io, io                            Description for io commands
    --field, field                      Description for field commands
    --manipulation, manipulation        Description for manipulation commands

    --masm, masm                        Prints the current masm Implementation
    --save, save <filename>             Saves the current state of the stack
    --end, end                          Ends the program

For more information about a specific command, use `help <command>`
";

pub const APP_VERSION: &'static str = "0.1.0";

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
        .override_help(APP_HELP)
}
