fn main() {
    let number1 = 7;

    // Must explicitly provide a boolean condition in an `if` statement
    if number1 < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number1 != 0 {
        println!("Number was something other than zero");
    }

    let number2 = 6;

    if number2 % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number2 % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number2 % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // `if` is an expression, so it can be used on the right side of a `let` statement to assign a value
    let condition = true;
    let number3 = if condition { 5 } else { 6 };

    // The following throws an error due to incompatible types: `let number3 = if condition { 5 } else { "six" };`
    // Rust must know what type number3 is definitively at compile time

    println!("The value of number is: {number3}");
}
