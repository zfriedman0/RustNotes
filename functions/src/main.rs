fn main() {
    println!("Hello, world!");

    another_function(42);
    print_labeled_measurements(5, 'h');

    let y = 6; // Statement, does not return the value the statement.

    let x = {
        let z = 3;
        z + 1
    };

    println!("The value of x is: {x}"); // Expression, returns the value of the expression block.

    let a = five(); // Use the value returned by five() to initialize a, same as `let a = 5;`

    println!("The value of a is: {a}");

    let b = plus_one(a);

    println!("The value of b is: {b}");
}

fn another_function(x: i32) { // MUST declare the type of each parameter in function signatures.
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 { // Specifies the type of the return value
    5 // This is the return value of the function
} // This function is an expression

fn plus_one(x: i32) -> i32 {
    x + 1
}
