use rust_masm::MidenProgram;
use rust_masm_cli::{app, APP_HELP, APP_VERSION, FIELD_HELP, IO_HELP, MANIPULATION_HELP};
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const HELP: [&'static str; 4] = ["help", "h", "--help", "-h"];
const VERSION: [&'static str; 4] = ["version", "-v", "-V", "--version"];

fn main() {
    let mut program = MidenProgram::new();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));

    writeln!(&mut stdout, "{}", APP_HELP).unwrap();

    'app_loop: loop {
        let mut input = String::new();

        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(189, 252, 206))));

        println!("\nstack : {:?}\n", program.get_stack());
        println!("ram : {:?}\n", program.get_ram_memory());

        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));

        print!("Command : ");

        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));

        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        println!("____________________________________________________\n");

        let mut args = vec!["rust-masm"];

        args.extend(input.split_whitespace());

        if args.len() == 1 {
            continue;
        }

        if args.len() == 2 {
            if HELP.contains(&args[1]) {
                let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                let _ = println!("{}", APP_HELP);
                continue;
            } else if VERSION.contains(&args[1]) {
                let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                let _ = println!("{}", APP_VERSION);
                continue;
            }
        }

        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
        if args.len() >= 3 && (HELP.contains(&args[2]) || HELP.contains(&args[1])) {
            let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
        }

        let matches = app().try_get_matches_from(args);

        match matches {
            Ok(matches) => match matches.subcommand() {
                Some(("end", _)) => {
                    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                    break 'app_loop;
                }
                Some(("field", _)) => {
                    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))) {
                        Ok(_) => {
                            let _ = writeln!(&mut stdout, "\n{}", FIELD_HELP);
                        }
                        Err(_) => {
                            println!("\n{}", FIELD_HELP);
                        }
                    }
                }
                Some(("manipulation", _)) => {
                    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))) {
                        Ok(_) => {
                            let _ = writeln!(&mut stdout, "\n{}", MANIPULATION_HELP);
                        }
                        Err(_) => {
                            println!("\n{}", MANIPULATION_HELP);
                        }
                    }
                }
                Some(("io", _)) => {
                    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))) {
                        Ok(_) => {
                            let _ = writeln!(&mut stdout, "\n{}", IO_HELP);
                        }
                        Err(_) => {
                            println!("\n{}", IO_HELP);
                        }
                    }
                }
                Some(("save", save_matches)) => {
                    let mut file: String = save_matches
                        .get_one::<String>("filename")
                        .unwrap()
                        .to_owned();
                    file.push_str(".masm");

                    program.save(&file);
                }
                Some(("masm", _)) => {
                    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))) {
                        Ok(_) => {
                            let _ = writeln!(&mut stdout, "\n{}", program.get_masm());
                        }
                        Err(_) => {
                            println!("\n{}", program.get_masm());
                        }
                    }
                }
                Some(("add", add_matches)) => {
                    if add_matches.contains_id("n") {
                        let n: u64 = *add_matches.get_one("n").unwrap();

                        program.add_n(n);
                    } else {
                        program.add();
                    }
                }
                Some(("sub", sub_matches)) => {
                    if sub_matches.contains_id("n") {
                        let n: u64 = *sub_matches.get_one("n").unwrap();

                        program.sub_n(n);
                    } else {
                        program.sub();
                    }
                }
                Some(("mul", mul_matches)) => {
                    if mul_matches.contains_id("n") {
                        let n: u64 = *mul_matches.get_one("n").unwrap();

                        program.mul_n(n);
                    } else {
                        program.mul();
                    }
                }
                Some(("div", div_matches)) => {
                    if div_matches.contains_id("n") {
                        let n: u64 = *div_matches.get_one("n").unwrap();

                        program.div_n(n);
                    } else {
                        program.div();
                    }
                }
                Some(("push", push_matches)) => {
                    if push_matches.contains_id("n") {
                        let n: Vec<u64> = push_matches.get_many("n").unwrap().copied().collect();

                        for n in n {
                            program.push(n);
                        }
                    }
                }

                Some(("inv", _)) => {
                    program.inv();
                }
                Some(("neg", _)) => {
                    program.neg();
                }
                Some(("pow2", _)) => {
                    program.pow2();
                }
                Some(("exp", exp_matches)) => {
                    if exp_matches.contains_id("n") {
                        let n: u64 = *exp_matches.get_one("n").unwrap();

                        program.exp_n(n);
                    } else {
                        program.exp();
                    }
                }
                Some(("and", _)) => {
                    program.and();
                }
                Some(("or", _)) => {
                    program.or();
                }
                Some(("movup", movup_matches)) => {
                    let n: usize = *movup_matches.get_one("index").unwrap();

                    program.movup_n(n);
                }
                Some(("movdn", movdn_matches)) => {
                    let n: usize = *movdn_matches.get_one("index").unwrap();

                    program.movdn_n(n);
                }
                Some(("movupw", movupw_matches)) => {
                    let n: usize = *movupw_matches.get_one("index").unwrap();

                    program.movupw_n(n);
                }
                Some(("movdnw", movdnw_matches)) => {
                    let n: usize = *movdnw_matches.get_one("index").unwrap();

                    program.movdnw_n(n);
                }
                Some(("padw", _)) => {
                    program.padw();
                }
                Some(("swap", swap_matches)) => {
                    if swap_matches.contains_id("index") {
                        let n: usize = *swap_matches.get_one("index").unwrap();

                        program.swap_n(n);
                    } else {
                        program.swap();
                    }
                }
                Some(("swapw", swapw_matches)) => {
                    if swapw_matches.contains_id("index") {
                        let n: usize = *swapw_matches.get_one("index").unwrap();

                        program.swapw_n(n);
                    } else {
                        program.swapw();
                    }
                }
                Some(("dup", dup_matches)) => {
                    if dup_matches.contains_id("index") {
                        let n: usize = *dup_matches.get_one("index").unwrap();

                        program.dup_n(n);
                    } else {
                        program.dup();
                    }
                }
                Some(("drop", _)) => {
                    program.drop();
                }

                Some(("mem_store", mem_store_matches)) => {
                    if mem_store_matches.contains_id("address") {
                        let n: u32 = *mem_store_matches.get_one("address").unwrap();

                        program.mem_store_n(n);
                    } else {
                        program.mem_store();
                    }
                }

                Some(("mem_load", mem_load_matches)) => {
                    if mem_load_matches.contains_id("address") {
                        let n: u32 = *mem_load_matches.get_one("address").unwrap();

                        program.mem_load_n(n);
                    } else {
                        program.mem_load();
                    }
                }

                Some(("mem_loadw", mem_loadw_matches)) => {
                    if mem_loadw_matches.contains_id("address") {
                        let n: u32 = *mem_loadw_matches.get_one("address").unwrap();

                        program.mem_load_w_n(n);
                    } else {
                        program.mem_load_w();
                    }
                }

                Some(("mem_storew", mem_storew_matches)) => {
                    if mem_storew_matches.contains_id("address") {
                        let n: u32 = *mem_storew_matches.get_one("address").unwrap();

                        program.mem_store_w_n(n);
                    } else {
                        program.mem_store_w();
                    }
                }

                Some(("xor", _)) => {
                    program.xor();
                }

                Some(("not", _)) => {
                    program.not();
                }

                Some(("eq", eq_matches)) => {
                    if eq_matches.contains_id("n") {
                        let n: u64 = *eq_matches.get_one("n").unwrap();

                        program.eq_n(n);
                    } else {
                        program.eq();
                    }
                }

                Some(("neq", eq_matches)) => {
                    if eq_matches.contains_id("n") {
                        let n: u64 = *eq_matches.get_one("n").unwrap();

                        program.neq_n(n);
                    } else {
                        program.neq();
                    }
                }

                Some(("lt", _)) => {
                    program.lt();
                }

                Some(("lte", _)) => {
                    program.lte();
                }

                Some(("gt", _)) => {
                    program.gt();
                }

                Some(("gte", _)) => {
                    program.gte();
                }

                Some(("is_odd", _)) => {
                    program.is_odd();
                }

                Some((_, _)) => {}
                None => {}
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
