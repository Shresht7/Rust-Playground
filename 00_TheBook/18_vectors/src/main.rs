// Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
// Vectors can only store values of the same type.

fn vectors() {
    // To create a new empty vector...
    let v: Vec<i32> = Vec::new();
    // Note that we had to use <i32> type annotation here to tell rust that this will be a vector of i32s.
    // Vectors are implemented using generics, we'll discuss generics later.

    // Alternatively, you can create a vector by providing initial values. Rust will infer the type from the given values. Fir thus we cause the vec! macro.
    let v = vec![1, 2, 3];

    // To add elements to the vector
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);

    // There are two ways to reference a value stored in a vector: by indexing or by using the `get` method.
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Rust provides these two ways to reference an element so you can choose how the program behaves when you try to use an index value outside the range of existing elements
    // When the get method is passed an index that is outside the vector, it returns None without panicking.
    // You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.
    // Your code will then have logic to handle having either Some(&element) or None

    // To iterate over the values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Vectors can only store values that are of the same type. This can be inconvenient; there are definitely use cases for needing to store a list of items of different types.
    // Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!
}

fn main() {
    vectors();
}
