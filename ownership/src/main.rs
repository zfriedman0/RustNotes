// Ownership Rules:
// Each value in Rust has an owner
// There can only be one owner at a time
// When the owner goes out of scope, the value will be dropped

fn main() {
    // Manages data allocated on the heap, able to store amount of text unknown at compile time
    // Allocates memory on the heap
    let mut s = String::from("hello");

    // `String::from` requests the memory needed to store a growable piece of text
    // As opposed to garbage collection or manual freeing, memory in Rust is automatically returned once the variable that owns it goes out of scope

    s.push_str(", world!");

    println!("{s}");

    let a = "hello"; // String literal, we know the contents at compile time.

    let s1 = String::from("hello");
    let s2 = s1; // `s1` gets moved into `s2`

    println!("{s2}, world!");

    let mut s3 = String::from("hello");
    s3 = String::from("ahoy");

    println!("{s3}, world!");

    // When a variable goes out of scope, Rust automatically calls `drop` to return memory

    let s4 = String::from("hello");
    let s5 = s4.clone(); // Create a deep copy of `s4`

    println!("s4 = {s4}, s5 = {s5}");

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}"); // Legal because integers have a known size at compile time

    // Rust has a special annotation called `Copy`
    // Can place this on types stored on the stack (like integers)
    // If a type implements `Copy`, variables that use it do not move.
    // Rust won't let us annotate a type with `Copy` if the type implements `Drop`

    takes_ownership(s);

    makes_copy(x);

    let s6 = gives_ownership();

    let s7 = String::from("hello");

    let s8 = takes_and_gives_back(s7);

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
