//  Ownership is a set of rules that governs how a rust program manages memory.

//  Each value in Rust has a variable that's called its owner
//  There can only be one owner at a time
//  When the owner goes out of scope, the value will be dropped

#[allow(dead_code)]
#[allow(unused_variables)]
fn test_s() {
    //  s is not valid here, it's not yet declared.
    let s = "hello"; //  s is valid from this point forward
                     //  do stuff with s
} //  Scope is now over and s is no longer valid

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let x = 5;
    let y = x;
    //  We first bind the value 5 to x; then make a copy of the value in x and bind it to y
    //  we now have two variables, x and y, and both equal 5.
    //  This is okay as integers are simple values with a known, fixed size and these two 5 values are can be pushed onto the stack

    let s1 = String::from("hello");
    let s2 = s1;
    //  This looks very similar but is in fact very different.
    //  When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length and the capacity that are on the stack
    //  We do not copy the data on the heap that the pointer refers to.
    //  If we were to copy the entire data, it would be very expensive in terms of runtime performance.
    //  Rust automatically calls drop when a variable goes out of scope. This is a problem as both s1 and s2 point to the same address in memory.
    //  If any of them were to go out of scope, it would invalidate the other. When both go out, they will try to free the same memory causing a double free error.
    //  To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid. i.e it moves the pointer from s1 to s2.
    //  Thus Rust doesn't need to free anything when s1 goes out of scope.

    //  The semantics for passing a value to a function are similar to those for assigning a value to a variable.
    //  Passing a variable to a function will move or copy, just as assignment does.

    let s = String::from("hello"); //  s comes into scope

    takes_ownership(s); //  s's value moves into the function and so is no longer valid here

    //  If we tried to use s here, Rust would throw a compile time error
    //  These static checks protect us from mistakes.

    let x = 5; //  x comes into scope

    makes_copy(x) //  x would move into the function, but i32 has the Copy trait, so its okay to still use x afterwards
} //  Here x and s go out of scope, since s was moved into takes_ownership() nothing special happens for s.

fn takes_ownership(some_string: String) {
    //  some_string comes into scope
    println!("{}", some_string);
} //  Here, some_string goes out of scope and drop is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    //  some_integer comes into scope
    println!("{}", some_integer);
} //  Here, some_integer goes out of scope. Nothing special happens

//  Returning values can also transfer ownership.
