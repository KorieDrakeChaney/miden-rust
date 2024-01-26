use clap::{value_parser, Arg, Command};

fn push() -> Command {
    Command::new("push")
        .about("Pushes a value onto the stack")
        .long_flag("push")
        .long_about("Pushes a value onto the stack")
        .args([Arg::new("n")
            .required(true)
            .value_parser(value_parser!(u64))
            .index(1)
            .num_args(1..)
            .help("The value to push onto the stack")])
}

fn mem_store() -> Command {
    Command::new("mem_store")
        .about("Stores a value in memory")
        .long_flag("mem_store")
        .long_about("Stores a value in memory")
        .args([Arg::new("address")
            .required(false)
            .value_parser(value_parser!(u32))
            .index(1)
            .default_missing_value("0")
            .num_args(1)
            .help("The address for the value to store in memory")])
}

fn mem_storew() -> Command {
    Command::new("mem_storew")
        .about("Stores 4 values in memory")
        .long_flag("mem_storew")
        .long_about("Stores 4 values in memory")
        .args([Arg::new("address")
            .required(false)
            .value_parser(value_parser!(u32))
            .index(1)
            .num_args(1)
            .help("The address to store the values in memory")])
}

fn mem_load() -> Command {
    Command::new("mem_load")
        .about("Loads a value from memory")
        .long_flag("mem_load")
        .long_about("Loads a value from memory")
        .args([Arg::new("address")
            .required(false)
            .value_parser(value_parser!(u32))
            .index(1)
            .num_args(1)
            .help("The address to load from memory")])
}

fn mem_loadw() -> Command {
    Command::new("mem_loadw")
        .about("Loads 4 values from memory")
        .long_flag("mem_loadw")
        .long_about("Loads 4 values from memory")
        .args([Arg::new("address")
            .required(false)
            .value_parser(value_parser!(u32))
            .index(1)
            .num_args(1)
            .help("The address to load from memory")])
}

fn loc_store() -> Command {
    Command::new("loc_store")
        .about("Stores a value in local memory")
        .long_flag("loc_store")
        .long_about("Stores a value in local memory")
        .args([Arg::new("address")
            .required(true)
            .value_parser(value_parser!(u16))
            .index(1)
            .num_args(1)
            .help("The address for the value to store in local memory")])
}

fn loc_storew() -> Command {
    Command::new("loc_storew")
        .about("Stores 4 values in local memory")
        .long_flag("loc_storew")
        .long_about("Stores 4 values in local memory")
        .args([Arg::new("address")
            .required(true)
            .value_parser(value_parser!(u16))
            .index(1)
            .num_args(1)
            .help("The address to store the values in local memory")])
}

fn loc_load() -> Command {
    Command::new("loc_load")
        .about("Loads a value from local memory")
        .long_flag("loc_load")
        .long_about("Loads a value from local memory")
        .args([Arg::new("address")
            .required(true)
            .value_parser(value_parser!(u16))
            .index(1)
            .num_args(1)
            .help("The address to load from local memory")])
}

fn loc_loadw() -> Command {
    Command::new("loc_loadw")
        .about("Loads 4 values from local memory")
        .long_flag("loc_loadw")
        .long_about("Loads 4 values from local memory")
        .args([Arg::new("address")
            .required(true)
            .value_parser(value_parser!(u16))
            .index(1)
            .num_args(1)
            .help("The address to load from local memory")])
}

pub fn commands() -> Vec<Command> {
    vec![
        push(),
        mem_store(),
        mem_storew(),
        mem_load(),
        mem_loadw(),
        loc_store(),
        loc_storew(),
        loc_load(),
        loc_loadw(),
    ]
}
