# Using Cargo
- We can create a project using `cargo new`.
	E.g. `cargo new hello_cargo`
- We can build a project using `cargo build`.
	Running `cargo build` for the first time causes Cargo to create a new file at the top level: 'Cargo.lock'
	This keeps track of versions of dependencies.
	Cargo manages the contents of this file for you.
- We can build and run a project in one step using `cargo run`.
	Compiles code and runs the resulting executable in one step.
	More convenient than usinf `cargo build` and using the file path to the executable.
- We can build a project without producing a binary to check for errors using `cargo check`.
	Much faster than `cargo build`.
	Useful for checking work to make sure code still compiles.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the *target/debug* directory.

An additional advantage of using Cargo is that the commands are the same no matter which operating system we're working on.

## Building for Release
When a project is ready for release, can use `cargo build --release` to compile with optimizations.
This will create an executable in *target/release* instead of *target/debug*
If benchmarking, be sure to run `cargo build --release` and benchmark with the executable in *target/release*.
