fn main() {
    let mut x = 5; // x is made mutable by the `mut` keyword, so its value can change.
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;

    // y gets shadowed.
    // The compiler will take this second value until the shadow is shadowed or scope ends.
    let y = y + 1;

    // Begin inner scope
    {
        // y gets shadowed again, but its scope ends outside of this block.
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    // End inner scope

    println!("The value of y is: {y}");

    // Another difference between `mut` and shadowing is that we can change the type of the value but reuse the same variable name.
    // We effectively create a new variable using the `let` keyword.
    let spaces = "     "; // Here `spaces` is a string.
    let spaces = spaces.len(); // Here it becomes a number/integer.

    // The following would throw a compile-time error:
    // let mut spaces = "     "; // Defines a mutable variable of type string
    // spaces = spaces.len() // This line tries to change the same variable's type, a no-no.
}