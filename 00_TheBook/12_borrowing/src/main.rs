//  A reference is like a pointer in that it's am address we can follow to access data stored at that address
//  that is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type.
//  References are denoted by an &. They allow you to refer to a value without taking ownership.

//  The opposite of referencing with & is dereferencing, which is accomplished by the dereferencing operator *.

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // &s1 is a reference to s1's value
    //  as &s1 is only a reference, calculate_length does not own it and will not drop it when it goes out of scope.
    //  When functions have references as parameters instead of actual values, we won't need to return the values
    //  in order to give back ownership, we never had it.
    //  This action of creating a reference is called borrowing.

    println!("The length of '{}' is {}", s1, len);

    //  Just as variables are immutable by default, so are references.
    //  You cannot modify something we have a reference to.
    //  We can explicitly make it mutable using the mut keyword

    let mut s2 = String::from("hello");
    change(&mut s2);
    //  The &mut s is a mutable reference.
    //  Mutable references have one big restriction, you can only have one mutable reference to a particular piece of data at a time.
    //  The restriction prevents multiple mutable references to the same data at the same time
    //  allows for mutation but in a very controlled fashion. Rust prevents data races at compile time.

    //  We also cannot have a mutable reference while we have an immutable one to the same value.
    //  Users of an immutable reference don't expect the value to suddenly change from under them.
    //  However, multiple immutable references are allowed because no one who is reading the data has the ability to affect anyone else
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
