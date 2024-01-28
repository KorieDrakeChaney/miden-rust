use rust_masm::{tokenize, EmptyProgram, Inputs, MidenProgram};

#[test]
fn test_inv() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.inv();

    program.print("test stack");
    program.save("programs/inv.masm");

    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_add() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.print("hello");
    program.push(5);
    program.add();
    program.print("testing add");

    program.save("programs/add.masm");
    program.print_masm();
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_add_n() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.add_n(10);

    program.save("programs/add_n.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_sub() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.push(5);
    program.sub();

    program.save("programs/sub.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_sub_n() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.sub_n(10);

    program.save("programs/sub_n.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_mul() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.push(5);
    program.mul();

    program.save("programs/mul.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_mul_n() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.mul_n(10);

    program.save("programs/mul_n.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_div() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.push(5);
    program.div();

    program.save("programs/div.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_div_n() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.div_n(10);

    program.save("programs/div_n.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_repeat() {
    let mut program = MidenProgram::new().with_inputs(Inputs::from_file("inputs/fibonacci.json"));

    let mut fib = MidenProgram::new();

    fib.swap();
    fib.dup_n(1);
    fib.add();

    program.repeat(10, || fib.get_operands());

    program.print_masm();

    program.save("programs/repeat.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_conditional() {
    let mut program = MidenProgram::new().with_inputs(Inputs::from_file("inputs/conditional.json"));
    program.push(3);
    program.push(5);

    program.adv_push(1);
    let mut if_program = MidenProgram::new();
    if_program.add();
    let mut else_program = MidenProgram::new();
    else_program.mul();
    program.if_else(|| if_program.get_operands(), || else_program.get_operands());

    program.save("programs/conditional.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_not() {
    let mut program = MidenProgram::new();

    program.push(1);
    program.not();

    program.save("programs/not.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_swap() {
    let mut program = MidenProgram::new();
    let mut program2 = MidenProgram::new();
    let mut pad_program = MidenProgram::new();

    pad_program.padw();

    program2.push(1);
    program.repeat(8, || pad_program.get_operands());
    program.repeat(7, || program2.get_operands());

    program.swapw_n(3);

    program.save("programs/swap.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_movup() {
    let mut program = MidenProgram::new();

    let mut pad_program = MidenProgram::new();

    pad_program.padw();

    program.repeat(4, || pad_program.get_operands());

    for i in 0..9 {
        program.push(i);
    }

    program.movup_n(3);

    program.save("programs/movup.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_movdn() {
    let mut program = MidenProgram::new();

    let mut pad_program = MidenProgram::new();

    pad_program.padw();

    program.repeat(4, || pad_program.get_operands());

    for i in 0..9 {
        program.push(i);
    }

    program.movdn_n(3);

    program.save("programs/movdn.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_movupw() {
    let mut program = MidenProgram::new();

    let mut pad_program = MidenProgram::new();

    pad_program.padw();

    program.repeat(4, || pad_program.get_operands());

    for i in 0..9 {
        program.push(i);
    }

    program.movupw_n(2);

    program.save("programs/movupw.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_movdnw() {
    let mut program = MidenProgram::new();

    let mut pad_program = MidenProgram::new();

    pad_program.padw();

    program.repeat(4, || pad_program.get_operands());

    for i in 0..9 {
        program.push(i);
    }

    program.movdnw_n(3);

    program.save("programs/movdnw.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_while() {
    let mut program = MidenProgram::new().with_inputs(Inputs::from_file("inputs/while.json"));

    let mut fib = MidenProgram::new();

    fib.swap();
    fib.dup_n(1);
    fib.add();

    let mut while_program = MidenProgram::new();

    while_program.add_program(|| fib.get_operands());

    while_program.dup();

    while_program.eq_n(89);

    let mut if_program = MidenProgram::new();
    if_program.push(0);
    let mut else_program = MidenProgram::new();
    else_program.push(1);

    while_program.if_else(|| if_program.get_operands(), || else_program.get_operands());

    program.adv_push(1);
    program.while_block(|| while_program.get_operands());

    program.save("programs/while.masm");
    program.print_masm();
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn nested_if_else() {
    let mut program =
        MidenProgram::new().with_inputs(Inputs::from_file("inputs/nested_if_else.json"));

    program.adv_push(1);

    let mut program_2 = MidenProgram::new();

    let mut if_program = MidenProgram::new();

    if_program.push(1);

    let mut else_program = MidenProgram::new();

    else_program.push(0);

    program_2.if_else(|| if_program.get_operands(), || else_program.get_operands());

    let mut program_3 = MidenProgram::new();

    program_3.push(1);

    let mut if_program_2 = MidenProgram::new();

    if_program_2.add_program(|| program_2.get_operands());

    if_program_2.add_program(|| program_2.get_operands());

    program_3.if_else(|| if_program_2.get_operands(), || program_2.get_operands());

    program.add_program(|| program_3.get_operands());

    program.print_masm();

    program.save("programs/nested_if_else.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn nested_repeat() {
    let mut program = MidenProgram::new();

    let mut program_2 = MidenProgram::new();

    let mut repeat_program = MidenProgram::new();

    repeat_program.push(1);

    program_2.repeat(2, || repeat_program.get_operands());

    let mut program_3 = MidenProgram::new();

    program_3.push(1);

    let mut repeat_program_2 = MidenProgram::new();

    repeat_program_2.add_program(|| program_2.get_operands());

    repeat_program_2.add_program(|| program_2.get_operands());

    program_3.repeat(2, || repeat_program_2.get_operands());

    program.add_program(|| program_3.get_operands());
    program.print_masm();

    program.print("test stack");
    program.save("programs/nested_repeat.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_mem_store() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.push(2);
    program.mem_store();
    program.push(2);
    program.mem_load();

    program.save("programs/mem_store.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_mem_store_n() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.mem_store_n(2);
    program.push(2);
    program.mem_load_n(2);

    program.save("programs/mem_store_n.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_mem_store_w() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.push(2);
    program.push(3);
    program.push(4);
    program.mem_store_w();

    let mut drop_program = MidenProgram::new();

    drop_program.drop();

    program.repeat(4, || drop_program.get_operands());

    program.push(4);

    program.mem_load_w();
    program.print_masm();
    program.print("test stack");
    program.save("programs/mem_store_w.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_mem_store_w_n() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.push(2);
    program.push(3);
    program.push(4);
    program.mem_store_w_n(4);

    let mut drop_program = MidenProgram::new();

    drop_program.drop();

    program.repeat(4, || drop_program.get_operands());

    program.mem_load_w_n(4);

    program.print("test stack");
    program.save("programs/mem_store_w_n.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_refresh() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.push(2);
    program.push(3);
    program.push(4);

    program.print("test stack");

    program.save("programs/refresh.masm");
    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_empty_program() {
    let mut add_program = EmptyProgram::new();

    add_program.add_n(5);

    let mut if_program = EmptyProgram::new();

    if_program.if_else(
        || add_program.get_operands(),
        || {
            let mut else_program = EmptyProgram::new();
            else_program.push(1);
            else_program.add_n(2);
            else_program.get_operands()
        },
    );

    let mut rand_program = EmptyProgram::new();

    rand_program.push(5);

    rand_program.push(1);

    rand_program.mem_store();

    rand_program.mem_load_n(1);

    rand_program.mul_n(2);

    rand_program.eq_n(10);

    let mut program = MidenProgram::new();

    program.add_program(|| rand_program.get_operands());

    program.add_program(|| if_program.get_operands());

    program.repeat(5, || {
        let mut repeat_program = EmptyProgram::new();

        repeat_program.exp_n(2);

        repeat_program.print("repeat_program");

        repeat_program.get_operands()
    });

    program.save("programs/empty_program.masm");
}

#[test]
fn while_block() {
    let mut program = MidenProgram::new();
    program.while_block(|| {
        let mut block = EmptyProgram::new();
        block.push(1);
        block.increment();
        block.dup();
        block.neq_n(10);
        block.print("loop");
        block.get_operands()
    });
}

#[test]
fn test_tokenizer() {
    let tokens = tokenize(
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
    ",
    );

    println!("{:?}", tokens);
}

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
    );

    match program {
        Ok(mut program) => {
            program.print_masm();
            program.save("programs/parse.masm");
            assert_eq!(Some(program.stack[0].into()), program.prove());
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[test]
fn test_prime() {
    let mut program = MidenProgram::parse_from_file_with_inputs(
        "examples/prime.masm",
        Inputs::from_file("inputs/prime.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/prime.masm");

    assert_eq!(Some(program.stack[0].into()), program.prove());
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
}

#[test]
fn test_game_of_life() {
    let mut program = MidenProgram::parse_from_file_with_inputs(
        "examples/game_of_life.masm",
        Inputs::from_file("inputs/game_of_life.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/game_of_life.masm");

    assert_eq!(Some(program.stack[0].into()), program.prove());
}

#[test]
fn test_catalan() {
    let mut program = MidenProgram::parse_from_file_with_inputs(
        "examples/catalan.masm",
        Inputs::from_file("inputs/catalan.json"),
    )
    .unwrap();

    program.print_masm();

    program.save("programs/catalan.masm");

    assert_eq!(Some(program.stack[0].into()), program.prove());
}
