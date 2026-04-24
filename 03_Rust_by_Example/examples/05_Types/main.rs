// Suppress all errors from casts which overflow
#![allow(overflowing_literals)]

// # 5.Types

fn main() {
    showcase_casting();
    showcase_literals();
    showcase_inference();
    showcase_aliasing();
}

// ## 5.1 Casting

// Rust provides no implicit type conversions (coercion) between primitive types. But, explicit type conversions (type-casting) can be performed using the `as` keyword.

fn showcase_casting() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal; // error: mismatched types

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.
    // let character = decimal as char; // error: non-primitive cast

    println!("Casting: {} -> {} -> '{}'", decimal, integer, character);

    // When casting any value to an unsigned type, T, T::MAX + 1 is added or subtracted until the value fits into the new type ONLY when the #![allow(overflowing_literals)]
    // lint is specified like above. Otherwise, there will be a compiler error.

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept, while the rest towards the most significant bit (MSB) are truncated.
    println!("1000 as a u8 is: {}", 1000 as u8);

    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus operator.
    println!("1000 mod 256 is: {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as first casting to the corresponding unsigned type.
    // if the most significant bit of the value is 1, then the value is negative.

    // Unless it already fits, of course
    println!("128 as i16 is: {}", 128 as i16);

    // In boundary case 128 value in 8-bit two's complement representation is -128
    println!("128 as i8 is: {}", 128 as i8);

    // Repeating the example above
    // 1000 as u8 is 232
    println!("1000 as i8 is: {}", 1000 as i8);

    // and the value of 232 in 8-bit two's complement representation is -24
    println!("232 as i8 is: {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast* when casting from a float to an integer.
    // If the floating point value exceeds the upper bound or is less than the lower bound, the returned value will be equal to the bound crossed.

    // 300.0 as u8 is 255
    println!("300.0 as u8 is: {}", 300.0_f32 as u8);

    // -100.0 as u8 is 0
    println!("-100.0 as u8 is: {}", (-100.0_f32) as u8);

    // NaN as u8 is 0
    println!("NaN as u8 is: {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided with unsafe methods, however, the results might overflow and
    // return **unsound values**. Use these methods wisely.
    unsafe {
        // 300.0 as u8 is 44
        println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());

        // -100.0 as u8 is 156
        println!("-100.0 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());

        // NaN as u8 is 0
        println!("NaN as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

// ## 5.2 Literals

// Numeric literals can be type annotated by adding the type after the number as a suffix. e.g. to specify that the literal `42` should have the type `i32`, write `42i32`.

// The type of unsuffixed numeric literals will depend on how they are used. If no constraints exist, the compiler will use `i32` for integers and `f64` for floating-point numbers.

fn showcase_literals() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of x in bytes: {}", std::mem::size_of_val(&x)); // size of x in bytes: 1
    println!("size of y in bytes: {}", std::mem::size_of_val(&y)); // size of y in bytes: 4
    println!("size of z in bytes: {}", std::mem::size_of_val(&z)); // size of z in bytes: 4
    println!("size of i in bytes: {}", std::mem::size_of_val(&i)); // size of i in bytes: 4
    println!("size of f in bytes: {}", std::mem::size_of_val(&f)); // size of f in bytes: 8
}

// ## 5.3 Inference

// The type inference engine is pretty smart. It does more than looking at the ttype of the value during an initialization.
// It also looks at how the variable is used afterwards to infer its type.

fn showcase_inference() {
    // Becuase of annotations, the compiler knows that `elem` is a `u8`
    let elem = 5u8;

    // Create an empty vector (a growable array)
    let mut vec = Vec::new();
    // At this point the compiler doesn't known the type of `vec`, it just knows that it's a vector of something `Vec<_>`.

    // Insert `elem` into `vec`
    vec.push(elem);
    // Now, the compiler can figure out that `vec` is a vector of `u8` because `elem` is a `u8`. So the type of `vec` is `Vec<u8>`.

    println!("vec: {:?}", vec); // vec: [5]
}

// ## 5.4 Aliasing

// The `type` keyword can be used to give a new name to an existing type.
// Types must have UpperCamelCase names, or the compiler will raise a warining. The exception to this rule are the primitive types like `usize`, `f32` etc.

fn showcase_aliasing() {
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    // `NanoSecond` = `Inch` = `U64` = `u64`
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    // Note that type aliases *don't* provide any extra type safety, because aliases are *not* new types.
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );

    // The main use of aliases is to reduce boilerplate, for example the `io::Result<T>` type is an alias for the `Result<T, io::Error>` type.
}
