//  Slices let you reference a contiguous sequence of elements in a collection
//  rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

#[allow(unused_variables)]
fn main() {
    let s = String::from("hello world!");
    //  A string slice is a reference to part of a String
    let hello = &s[0..5];
    let world = &s[6..11];
    //  Rather than reference to the entire string, hello refers to only a portion.
    //  We create slices using a range within brackets by specifying [start..end].

    //  Slices exist for other types too
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}

#[allow(dead_code)]
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
