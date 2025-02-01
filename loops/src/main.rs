fn main() {
    // Rust provides three kinds of loops: `loop`, `while`, and `for`.
    // The `loop` keyword tells Rust to execute a block of code forever or until you explicitly tell it to stop.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    println!();

    let mut count = 0;
    // Can specify a loop label that can be used with `break` or `continue` to specify that those keywords apply to the labeled loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // Exits the outer, labeled loop
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let mut number = 3;

    println!();

    // `while` loops execute while a condition is true
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Can also use a `while` loop to loop over elements of a collection.
    // This approach is error-prone
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("\nUsing `while` to loop through elements:");

    while index < 5 {
        println!("The value is {}", a[index]);

        index += 1;
    }

    println!("\nUsing `for` to loop through elements:");

    // A more concise way is to use a `for` loop
    for element in a {
        println!("The value is: {element}");
    }

    println!();

    // Can use `for` to loop a specified number of times
    // `rev()` reverses the range
    // Range (1..4) is non-inclusive on the upper bound, can replace with (1..=3) for same effect
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
