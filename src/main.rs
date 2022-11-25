use asi_sandbox::world::world::*;

fn tests() {
    let _test_cell = Cell { id: 0 };

    let _test_stack = CellStack {
        id: 0,
        stack: vec![_test_cell],
    };
}

fn main() {
    tests();

    println!("It works!");
}
