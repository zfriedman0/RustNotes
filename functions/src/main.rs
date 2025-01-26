fn main() {
    println!("Hello, world!");

    another_function(42);
    print_labeled_measurements(5, 'h')
}

fn another_function(x: i32) { // MUST declare the type of each parameter in function signatures.
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
