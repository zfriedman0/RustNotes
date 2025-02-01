use rand::Rng; // `Rng` is a trait
use std::cmp::Ordering;
use std::io; // Input/output library, allows user input.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // start..=end, inclusive on upper and lower bounds

    loop {
        println!("Please input your guess.");

        // `let` declares a variable
        // E.g. let apples = 5;

        // In Rust, variables are immutable by default.
        // `mut` makes the variable mutable
        let mut guess = String::new(); // String is a type provided by the standard library, creates a new empty string.

        // ::new indicates that `new` is an associated function of the String type.
        // An associated function is a function that is implemented on a type.

        io::stdin() // Could rewrite as std::io::stdin if io library had not been imported in preamble, returns an instance of std::io::Stdin.
            .read_line(&mut guess) // Takes what user writes in stdin and assigns to guess. `&` indicates a reference.
            .expect("Failed to read line"); // Error handling, displays custom message on failure.

        let guess: u32 = match guess.trim().parse() {
            // Shadowing, do this to convert variable from one type to another
            Ok(num) => num,
            Err(_) => continue, // _ is a catch-all value, matches all Err values. `continue` tells program to go to next iteration of the loop.
        };

        println!("You guessed: {}", guess); // Curly brackets are variable placeholder.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // A match expression is made up of arms
            Ordering::Greater => println!("Too big!"), // These are arms
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
