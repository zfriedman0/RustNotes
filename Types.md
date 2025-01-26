# Data Types
Every value in Rust is of a certain *data type*.
There are two data type subsets: **scalar** and **compound**.

Rust is statically typed, so the compiler must know the types of all variables at compile time.
The compiler can usually infer a value's type based on how it is used.
In cases where it cannot, we must **annotate**:

`let guess: u32 = "42".parse().expect("Not a number!")`

## Scalar Types
- Represents a single value.
- Four primary scalar types:
    + integers
    + floating-point numbers
    + booleans
    + characters

### Integer Types
A number without a fractional component.

| **Length** | **Signed** | **Unsigned** |
| ---------- | ---------- | ------------ |
| 8-bit      | `i8`       | `u8`         |
| 16-bit     | `i16`      | `u16`        |
| 32-bit     | `i32`      | `u32`        |
| 64-bit     | `i64`      | `u64`        |
| 128-bit    | `i128`     | `u128`       |
| arch       | `isize`    | `usize`      |

Each variant can be either signed or unsigned.
    
- Refers to whether it's possible for a number to be negative.
    - Signed numbers are stored using two's complement representation.

Signed variants can store numbers from -(2^(n-1)) to (2^(n-1)-1) inclusive, where *n* is the number of bits that variant uses.
    
- `i8` can store from -(2^(7)) to ((2^7)-1) numbers (-128 to 127).

Unsigned variants can store numbers from 0 to ((2^n)-1).
    
- `u8` can store from 0 to ((2^8)-1) numbers (0 to 255).

`isize` and `usize` depend on the architecture of the system the program is running on.
    
- 64 bits if on 64-bit architecture, 32 bits if on 32-bit architecture.

Integer literals can be written in any of the following forms:

| **Number literals** | **Example**   |
| ------------------- | ------------- |
| Decimal             | `98_222`      |
| Hex                 | `0xff`        |
| Octal               | `0o77`        |
| Binary              | `0b1111_0000` |
| Byte (`u8` only)    | `b'A'`        |

Number literals that can be multiple numeric types allow a type suffix, such as `57u8`, to designate the type.

Number literals can also use a `_` as a visual separator to make it easier to read.
    
- `1_000` will have the same value as `1000`.

If you're unsure what type to use, Rust's defaults are a good place to start: integer types default to `i32`.
    
- The primary situation in which you'd use `isize` and `usize` is when indexing some sort of collection.

### Floating-Point Types
Rust has two primitive types for floating-point numbers, which are numbers with decimal points.
    
- `f32` and `f64`, which are 32 bits and 64 bits in size respectively.

`f64` is default, since it is roughly the same speed as `f32` and offers more precision.

All floating-point types are signed.

    fn main() {
        let x = 2.0; // f64
        let y: f32 = 3.0; // f32
    }

`f32` is a single-precision float, `f64` has double precision.

### Numeric Operations
Rust supports all basic mathematical operations for number types: addition, subraction, multiplication, division, and remainder (modulus).

Integer division truncates toward zero to the nearest integer.

    fn main() {
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1

        // remainder
        let remainder = 43 % 5;
    }

### Boolean Type
A Rust boolean has two possible values: `true` or `false`.

Booleans are one byte in size.

A boolean is specified using the `bool` keyword.

    fn main() {
        let t = true;

        let f: bool = false; // with explicit type annotation
    }

The main way to use booleans is through conditionals, such as an `if` expression.

### Character Type
Rust's `char` is the most primitive alphabetic type.

    fn main() {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    }

We specify `char` literals with single quotes.
    
- String literals use double quotes.

`char` type is four bytes in size and represents a unicode scalar value.
    
- Can represent much more than ASCII. Accented letters; Chinese, Japanese, Korean characters; emoji; and zero-width spaces are all valid.

## Compound Types
*Compound types* can group multiple values into one type. Rust has two primitive compound types: **tuples** and **arrays**.

### Tuple Type
A tuple is a general way of grouping together a number of values with a variety of types into one compound type.

Tuples have fixed length: they cannot grow or shrink in size once declared.

Create a tuple by declaring a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types do not need to match.

    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1); // optional type annotation included
    }

To get the individual values out of a tuple, we can use pattern matching to destructure a tuple:

    fn main() {
        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;

        println("The value of y is: {y}");
    }

We can also access a tuple element directly using `.` followed by the index of the value we want to access.

    fn main() {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
    }

The tuple without any elements has a special name: the **unit**. This value and its corresponding type are both written `()` and represent an empty value or an empty return type.
    
- Expressions implicitly return the unit value if they don't return any other value.

### Array Type
Unlike tuples, every element of an array must have the same type.

Arrays in Rust have a fixed length.

We write values in an array as a comma-separated list inside square brackets.

    fn main() {
        let a = [1, 2, 3, 4, 5];
    }

Arrays are useful when you want your data allocated on the stack rather than the heap, or when you want to ensure you always have a fixed number of elements.

Useful when you know the number of elements will not need to change.
    
- E.g. the names of the months:

    fn main() {
        let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    }

Write an array's type using square brackets with the type of each element, a semicolon, and then the number of elements in the array:

    fn main() {
        let a: [i32; 5] = [1, 2, 3, 4, 5];
    }

Can also initialize an array to contain the same value for each element by specifying the initial value, semicolon, then length:

    fn main() {
        let a = [3; 5];
    }

Array `a` will contain 5 elements that will all be set to the value 3 initially. Same as writing `let a = [3, 3, 3, 3, 3]`.

#### Accessing Array Elements
An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.

Access the elements of an array using indexing:

    fn main() {
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
    }
