use clap::{value_parser, Arg, Command};

fn masm() -> Command {
    Command::new("masm")
        .about("Prints the current masm Implementation")
        .long_flag("masm")
        .long_about("Prints the current masm Implementation")
}

fn save() -> Command {
    Command::new("save")
        .about("Saves the current state of the stack")
        .long_flag("save")
        .long_about("Saves the current state of the stack")
        .arg(
            Arg::new("filename")
                .required(false)
                .value_parser(value_parser!(String))
                .index(1)
                .default_value("stack")
                .num_args(1)
                .help("The name of the file to save the stack to"),
        )
}

fn end() -> Command {
    Command::new("end")
        .about("Ends the program")
        .long_flag("end")
        .long_about("Ends the program")
}

pub fn commands() -> Vec<Command> {
    vec![masm(), save(), end()]
}
