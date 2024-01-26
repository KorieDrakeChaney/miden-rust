use clap::{value_parser, Arg, Command};

fn add() -> Command {
    Command::new("add")
        .about("Adds two values")
        .long_flag("add")
        .long_about(
            "Pops two values off the stack, adds them, and pushes the result onto the stack",
        )
        .arg(
            Arg::new("n")
                .required(false)
                .value_parser(value_parser!(u64))
                .index(1)
                .num_args(1)
                .help("The first value to add"),
        )
}

fn sub() -> Command {
    Command::new("sub")
        .about("Subtracts two values")
        .long_flag("sub")
        .long_about(
            "Pops two values off the stack, subtracts them, and pushes the result onto the stack",
        )
        .arg(
            Arg::new("n")
                .required(false)
                .value_parser(value_parser!(u64))
                .index(1)
                .num_args(1)
                .help("The first value to subtract"),
        )
}

fn mul() -> Command {
    Command::new("mul")
        .about("Multiplies two values")
        .long_flag("mul")
        .long_about(
            "Pops two values off the stack, multiplies them, and pushes the result onto the stack",
        )
        .arg(
            Arg::new("n")
                .required(false)
                .value_parser(value_parser!(u64))
                .index(1)
                .num_args(1)
                .help("The first value to multiply"),
        )
}

fn div() -> Command {
    Command::new("div")
        .about("Divides two values")
        .long_flag("div")
        .long_about(
            "Pops two values off the stack, divides them, and pushes the result onto the stack",
        )
        .arg(
            Arg::new("n")
                .required(false)
                .value_parser(value_parser!(u64))
                .index(1)
                .num_args(1)
                .help("The first value to divide"),
        )
}

fn neg() -> Command {
    Command::new("neg")
        .about("Negates the top value")
        .long_flag("neg")
        .long_about(
            "Pops the top value off the stack, negates it, and pushes the result onto the stack",
        )
}

fn inv() -> Command {
    Command::new("inv")
        .about("Inverts the top value")
        .long_flag("inv")
        .long_about(
            "Pops the top value off the stack, inverts it, and pushes the result onto the stack",
        )
}

fn pow2() -> Command {
    Command::new("pow2")
        .about("Raises 2 to the power of the top value")
        .long_flag("pow2")
        .long_about("Pops the top value off the stack, raises 2 to the power of it, and pushes the result onto the stack")
}

fn exp() -> Command {
    Command::new("exp")
        .about("Raises a value to a value")
        .long_flag("exp")
        .long_about(
            "Pops two values off the stack, first value is the exponent, second value is the base",
        )
        .arg(
            Arg::new("n")
                .required(false)
                .value_parser(value_parser!(u64))
                .index(1)
                .num_args(1)
                .help("The exponent"),
        )
}

fn and() -> Command {
    Command::new("and")
        .about("Performs a bitwise and on two binary values")
        .long_flag("and")
        .long_about(
            "Assumes the top two values on the stack are binary values, performs a bitwise and on them, and pushes the result onto the stack",
        )
}

fn or() -> Command {
    Command::new("or")
        .about("Performs a bitwise or on two binary values")
        .long_flag("or")
        .long_about(
            "Assumes the top two values on the stack are binary values, performs a bitwise or on them, and pushes the result onto the stack",
        )
}

fn xor() -> Command {
    Command::new("xor")
        .about("Performs a bitwise xor on two binary values")
        .long_flag("xor")
        .long_about(
            "Assumes the top two values on the stack are binary values, performs a bitwise xor on them, and pushes the result onto the stack",
        )
}

fn not() -> Command {
    Command::new("not")
        .about("Performs a bitwise not on a binary value")
        .long_flag("not")
        .long_about(
            "Assumes the top value on the stack is a binary value, performs a bitwise not on it, and pushes the result onto the stack",
        )
}

fn eq() -> Command {
    Command::new("eq")
        .about("Checks if two values are equal")
        .long_flag("eq")
        .long_about(
            "Pops two values off the stack, checks if they are equal, and pushes the result onto the stack",
        ).arg(
            Arg::new("n")
                .required(false)
                .value_parser(value_parser!(u64))
                .index(1)
                .num_args(1)
                .help("The first value to check"),
        )
}

fn lt() -> Command {
    Command::new("lt")
        .about("Checks if the first value is less than the second value")
        .long_flag("lt")
        .long_about(
            "Pops two values off the stack, checks if the first value is less than the second value, and pushes the result onto the stack",
        )
}

fn gt() -> Command {
    Command::new("gt")
        .about("Checks if the first value is greater than the second value")
        .long_flag("gt")
        .long_about(
            "Pops two values off the stack, checks if the first value is greater than the second value, and pushes the result onto the stack",
        )
}

fn lte() -> Command {
    Command::new("lte")
        .about("Checks if the first value is less than or equal to the second value")
        .long_flag("lte")
        .long_about(
            "Pops two values off the stack, checks if the first value is less than or equal to the second value, and pushes the result onto the stack",
        )
}

fn gte() -> Command {
    Command::new("gte")
        .about("Checks if the first value is greater than or equal to the second value")
        .long_flag("gte")
        .long_about(
            "Pops two values off the stack, checks if the first value is greater than or equal to the second value, and pushes the result onto the stack",
        )
}

fn neq() -> Command {
    Command::new("neq")
        .about("Checks if two values are not equal")
        .long_flag("neq")
        .long_about(
            "Pops two values off the stack, checks if they are not equal, and pushes the result onto the stack",
        )
        .arg(
            Arg::new("n")
                .required(false)
                .value_parser(value_parser!(u64))
                .index(1)
                .num_args(1)
                .help("The first value to check"),
        )
}

pub fn commands() -> Vec<Command> {
    vec![
        add(),
        sub(),
        mul(),
        div(),
        neg(),
        inv(),
        pow2(),
        exp(),
        and(),
        or(),
        xor(),
        not(),
        eq(),
        lt(),
        gt(),
        lte(),
        gte(),
        neq(),
    ]
}
