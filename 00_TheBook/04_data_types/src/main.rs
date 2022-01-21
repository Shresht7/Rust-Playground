//  This weird syntax disables `dead_code` and `unused_variables` warnings.
#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    //  Rust is a statically typed language, which means it must know the types of all variables at compile time.
    //  The compiler can usually infer what type we want to use based on the value but even this happens before runtime.

    //  SCALAR TYPES
    //  ============

    //  A scalar type represents a single value. e.g `integers`, `floating-point numbers`, `booleans`, and `characters`

    // INTEGERS
    // --------

    //  An integer is a number without a fractional component.
    let x = 42;

    //  Integers have many variants. They can be signed (i) or unsigned (u).
    //  Size ranges from 8-bit to 128-bit. `isize` and `usize` types depend on the computer architecture (32-bit or 64-bit).

    //  Number literals can use `_` as a visual separator to make the number easier to read.
    const ONE_BILLION: u32 = 1_000_000_000;

    //  Trying to assign a value beyond the variables range will cause an integer overflow.
    //  Rust will panic if this happens in debug mode. However, in release builds, it will wrap around to the minimum values.
    //  in case of `u8` (0 - 255), 256 will become 0, 257 will become 1.

    //  FLOATS
    //  ------

    //  Rust has two primitive types for floating-points numbers. f32 and f64. On modern architecture, f64 is roughly the same speed and twice the precision.

    //  Rust supports the basic mathematical operations
    //  Addition
    let sum = 5 + 10;
    //  Subtraction
    let difference = 95.5 - 4.3;
    //  Multiplication
    let product = 4 * 30;
    //  Division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; //  Both 2 and 3 are integers (i32). So the result is also i32 (disregards decimal points)

    //  Remainder
    let remainder = 43 % 5;

    //  BOOLEANS
    //  --------

    //  A boolean has two possible values `true` and `false`
    //  Booleans are only one byte in size.

    let t = true;
    let f: bool = false;

    //  CHARACTER
    //  ---------

    //  char is Rust's most primitive character type
    let c = 'z';
    //  Rust's char is 4 bytes in size and represents a Unicode Scalar Value.

    //  COMPOUND TYPES
    //  ==============

    //  Compound types can group multiple values into one type. e.g. tuples and arrays

    //  TUPLE
    //  -----

    //  A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    //  Tuples have fixed length.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; //  <--- 🎉 Destructuring!

    //  Values of a tuple can also be accessed using the dot notation. tup.1, tup.0 etc.

    println!("The values of x: {}, y: {} z:{}", x, y, z);

    //  The tuple without any values (), is a special type that has only one value.
    //  Expressions implicitly return the unit value if they don't return any other value.

    //  ARRAYS
    //  ------

    //  Arrays are also a collection of multiple values. Every element must have the same type.
    //  The length of an array is fixed like tuples. This is different from many other languages.
    let arr = [1, 2, 3, 4, 5];

    //  The array type notation has the following format `: [dataType; length]`

    //  Indexing arrays works the way you'd expect.
    let first = arr[0];
    let second = arr[1];

    let ten_zeroes = [0; 10]; //  Creates an array of length 10 where all elements are 0

    //  A dynamic array is called a vector. It is allowed to grow and shrink in size.
}
