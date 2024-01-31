use rust_masm::{EmptyProgram, Inputs, MidenProgram, Proc};

#[test]
fn test_parse() {
    let program = MidenProgram::parse(
        "
        proc.MasmFromRust.9095
        push.1
        push.2
        push.3
        push.4
        add
        add
        add
        push.20
        push.30
        push.40
        mul.2
        mul.2
        push.5
        push.4
        push.3
        push.2
        loc_store.1
        loc_storew.5
        loc_storew.9
        loc_storew.90
        loc_storew.909
        loc_storew.9094
        loc_storew.906
        loc_storew.9069
        add
        mul
        mul.2
        add
        mul.2
        exp.2
        add.10
        end

        begin
        exec.MasmFromRust
        end
    ",
    )
    .unwrap();

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![211610, 30, 20, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
}

#[test]
fn test_prime() {
    let program = MidenProgram::parse_from_file_with_inputs(
        "examples/prime.masm",
        Inputs::from_file("inputs/prime.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/prime.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![
            229, 227, 223, 211, 199, 197, 193, 191, 181, 179, 173, 167, 163, 157, 151, 149, 139,
            137, 131, 127, 113, 109, 107, 103, 101, 97, 89, 83, 79, 73, 71, 67, 61, 59, 53, 47, 43,
            41, 37, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0
        ],
    );
}

#[test]
fn test_add_parse() {
    let mut program = MidenProgram::parse(
        "
        begin
          push.1.2.3.4.5
          u32checked_mod
        end",
    )
    .unwrap();

    program.print("add 5 and 1");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
}

#[test]
fn test_parse_3() {
    let mut simple_miden_program = MidenProgram::parse(
        "
        begin
        push.1.2.3.4.5.6
        add
        add
        add.1
        dup.1
        swap
        swap.2
        swap.3
        swap.4
        mul
        mul.5
        exp.2
        push.1.123.12.312.31.12.312.21
        end
        ",
    )
    .unwrap();

    simple_miden_program.add();
    simple_miden_program.exp_n(4);

    simple_miden_program.print_masm();
    assert_eq!(
        simple_miden_program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![
            12296370321,
            12,
            31,
            312,
            12,
            123,
            1,
            225,
            16,
            3,
            2,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0
        ],
    );
}

#[test]
fn test_game_of_life() {
    let program = MidenProgram::parse_from_file_with_inputs(
        "examples/game_of_life.masm",
        Inputs::from_file("inputs/game_of_life.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/game_of_life.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![
            1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ],
    );
}

#[test]
fn test_catalan() {
    let program = MidenProgram::parse_from_file_with_inputs(
        "examples/catalan.masm",
        Inputs::from_file("inputs/catalan.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/catalan.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![4862, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
}

#[test]

fn test_collatz() {
    let program = MidenProgram::parse_from_file_with_inputs(
        "examples/collatz.masm",
        Inputs::from_file("inputs/collatz.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/collatz.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![132, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
}

#[test]
fn test_comparison() {
    let program = MidenProgram::parse_from_file_with_inputs(
        "examples/comparison.masm",
        Inputs::from_file("inputs/comparison.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/comparison.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_conditional() {
    let program = MidenProgram::parse_from_file_with_inputs(
        "examples/conditional.masm",
        Inputs::from_file("inputs/conditional.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/conditional.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn fibonacci() {
    let program = MidenProgram::parse_from_file_with_inputs(
        "examples/fibonacci.masm",
        Inputs::from_file("inputs/fibonacci.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/fibonacci.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![
            11112721240812633725,
            16245143635561662896,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    );
}

#[test]
fn test_matrix_mul() {
    //todo: fix this
    let program = MidenProgram::parse_from_file_with_inputs(
        "examples/matrix_mul.masm",
        Inputs::from_file("inputs/matrix_mul.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/matrix_mul.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_adv_pipe() {
    let program = MidenProgram::parse_from_file_with_inputs(
        "examples/adv_pipe.masm",
        Inputs::from_file("inputs/adv_pipe.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/adv_pipe.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![7, 6, 5, 4, 3, 2, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
}

#[test]
fn test_masm() {
    let program = MidenProgram::parse_from_file("examples/errors.masm").unwrap();

    program.print_masm();

    program.save("programs/example.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![2, 2, 2, 2, 2, 2, 2, 2, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_error_handling() {
    let program = MidenProgram::parse(
        "
        begin
            push.1
            push.2
            and 
        end
        ",
    )
    .unwrap();

    program.print_masm();

    program.save("programs/test.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_conditional_manipulation() {
    let program = MidenProgram::parse(
        "
        begin
            push.1
            push.2
            eq
            push.1
            push.2
            eq
            push.1
            push.2
            eqw
            push.1
            push.2
            lt
            push.1
            push.2
            gt
            push.1
            push.2
            cswap
            push.1
            push.2
            cswapw
            push.1
            push.2
            cdrop
            push.1
            push.2
            cdropw
        end
        ",
    )
    .unwrap();

    program.print_masm();

    program.save("programs/conditional_manipulation.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![
            2, 1, 2, 1, 2, 1, 2, 1, 0, 1, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0
        ]
    );
}

#[test]
fn test_hex() {
    let program = MidenProgram::parse(
        "
        begin
            push.0x00001234.0x00005678.0x00009012.0x0000abcd
            push.0x341200000000000078560000000000001290000000000000cdab000000000000
            push.4660.22136.36882.43981
        end
        ",
    )
    .unwrap();

    program.print_masm();

    program.save("programs/hex.masm");

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![
            43981, 36882, 22136, 4660, 43981, 36882, 22136, 4660, 43981, 36882, 22136, 4660, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
    )
}

#[test]
fn test_test_test() {
    let mut procedure = Proc::new("whileExample");
    procedure.push(1);
    let mut while_program = EmptyProgram::new();
    while_program.increment();
    while_program.dup();
    while_program.neq_n(10);
    procedure.while_block(&mut while_program);
    let mut program = MidenProgram::new();
    program.push(1);
    program.add_proc(procedure);
    program.exec("whileExample");

    program.print_masm();

    assert_eq!(
        program
            .stack
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<u64>>(),
        vec![10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}
