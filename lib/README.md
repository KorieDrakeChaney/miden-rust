# Rust MASM

## A simple Rust library for generating MASM code

### Usage

```rust
use rust_masm::MidenProgram;

fn main() {
    let mut program = MidenProgram::new();

    program.push(5);
    program.push(1);
    program.add();
}
```

This will generate the following MASM code:

```masm
begin
    push 5
    push 1
    add
end
```

You can do more complex things like:

```rust
use rust_masm::{MidenProgram, EmptyProgram};

fn main() {
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

        repeat_program.get_operands()
    });
}
```

This will generate the following MASM code:

```masm
begin
	push.5
	push.1
	mem_store
	mem_load.1
	mul.2
	eq.10
	if.true
		add.5
	else
		push.1
		add.2
	end
	repeat.5
		exp.2
	end
end

#stack output : [3273349785757477523, 0, 0, 0, 0, 0, 0, 0]
```

### Seeing your stack transform

You can see how your stack transforms by using the `print` method:

```rust
use rust_masm::{MidenProgram};

fn main() {
    let mut program = MidenProgram::new();

    program.push(5);

    program.print("push 5");

    program.push(1);

    program.print("push 1");

    program.add();

    program.print("add");
}
```

Which is beneficial for debugging

# Parse from MASM to Rust

You can also parse MASM code into Rust code:

```rust
use rust_masm::{MidenProgram};

fn main() {
    let mut program = MidenProgram::parse("
	begin
        	push.5
        	push.1
        	add
    end
    "
    ).unwrap();

    program.print("add 5 and 1");
}
```
