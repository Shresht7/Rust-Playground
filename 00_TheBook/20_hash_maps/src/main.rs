// The type HashMap<K, V> stores a mapping of keys of type K to type V, using a hashing function, which determines how it places
// the keys and values into memory. In other languages, this is often called a hash, map, object, hash-table, dictionary or associative array.
// Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.

// Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

use std::collections::HashMap;

fn hash_maps() {
    // One way to create a hash map is to use new and to add elements with insert.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // We can get a value out of the hash map by providing its key to the get method,
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // We can iterate over each key-value pair like we do vectors
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values

    // If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.

    // t’s common to check whether a particular key already exists in the hash map with a value and then to take the following actions:
    // if the key does exist in the hash map, the existing value should remain the way it is; if the key doesn’t exist, insert it and a value for it.

    // Hash maps have a special API for this called entry that takes the key you want to check as a parameter.
    // The return value of the entry method is an enum called Entry that represents a value that might or might not exist.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists,
    // and if not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    // This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.
}

fn main() {
    fn hash_maps();
}
