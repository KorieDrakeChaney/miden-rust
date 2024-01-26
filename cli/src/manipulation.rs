use clap::{value_parser, Arg, Command};

fn drop() -> Command {
    Command::new("drop")
        .about("Drops first value from stack")
        .long_flag("drop")
        .long_about("Drops first value from stack")
}

fn dup() -> Command {
    Command::new("dup")
        .about("Duplicates first value on stack")
        .long_flag("dup")
        .long_about("Duplicates first value on stack")
        .arg(
            Arg::new("index")
                .required(false)
                .value_parser(value_parser!(usize))
                .index(1)
                .default_value("0")
                .num_args(1)
                .help("Index of value, only 0-15 are valid"),
        )
}

fn swap() -> Command {
    Command::new("swap")
        .about("Swaps first two values on stack")
        .long_flag("swap")
        .long_about("Swaps first two values on stack")
        .arg(
            Arg::new("index")
                .required(false)
                .value_parser(value_parser!(usize))
                .index(1)
                .default_value("1")
                .num_args(1)
                .help("Index of value, only 1-15 are valid"),
        )
}

fn swapw() -> Command {
    Command::new("swapw")
        .about("Swaps 0,1,2,3 with n,n+1,n+2,n+3")
        .long_flag("swapw")
        .long_about("Swaps 0,1,2,3 with n,n+1,n+2,n+3")
        .arg(
            Arg::new("index")
                .required(false)
                .value_parser(value_parser!(usize))
                .index(1)
                .default_value("1")
                .num_args(1)
                .help("Index of value, only 1-3 are valid"),
        )
}

fn padw() -> Command {
    Command::new("padw")
        .about("Pads stack with 4 0s")
        .long_flag("padw")
        .long_about("Pads stack with 4 0s")
}

fn movup() -> Command {
    Command::new("movup")
        .about("Moves value at index n to index 0")
        .long_flag("movup")
        .long_about("Moves value at index n to index 0")
        .arg(
            Arg::new("index")
                .required(true)
                .value_parser(value_parser!(usize))
                .index(1)
                .num_args(1)
                .help("Index of value, only 1-15 are valid"),
        )
}

fn movupw() -> Command {
    Command::new("movupw")
        .about("Moves values at index n,n+1,n+2,n+3 to 0,1,2,3")
        .long_flag("movupw")
        .long_about("Moves values at index n,n+1,n+2,n+3 to 0,1,2,3")
        .arg(
            Arg::new("index")
                .required(true)
                .value_parser(value_parser!(usize))
                .index(1)
                .num_args(1)
                .help("Index of value, only 2-3 are valid"),
        )
}

fn movdn() -> Command {
    Command::new("movdn")
        .about("Moves value at index 0 to index n")
        .long_flag("movdn")
        .long_about("Moves value at index 0 to index n")
        .arg(
            Arg::new("index")
                .required(true)
                .value_parser(value_parser!(usize))
                .index(1)
                .num_args(1)
                .help("Index of value, only 2-15 are valid"),
        )
}

fn movdnw() -> Command {
    Command::new("movdnw")
        .about("Moves values at index 0,1,2,3 to n,n+1,n+2,n+3")
        .long_flag("movdnw")
        .long_about("Moves values at index 0,1,2,3 to n,n+1,n+2,n+3")
        .arg(
            Arg::new("index")
                .required(true)
                .value_parser(value_parser!(usize))
                .index(1)
                .num_args(1)
                .help("Index of value, only 2-3 are valid"),
        )
}

pub fn commands() -> Vec<Command> {
    vec![
        drop(),
        dup(),
        swap(),
        swapw(),
        padw(),
        movup(),
        movupw(),
        movdn(),
        movdnw(),
    ]
}
