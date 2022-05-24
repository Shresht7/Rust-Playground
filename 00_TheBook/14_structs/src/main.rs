//  A struct, or structure, is a custom data type that lets you name and package together multiple related
//  values that make up a meaningful group. A struct is like an object's data attributes.

//  Struts are similar to tuples, the pieces of structs can be different types. Unlike tuples, you'll name
//  each piece of data so it's clear what the values mean. As a result, structs are more flexible than tuples
//  you don't have to rely on the order of the data to specify or access the values of an instance

#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(unused_variables)]
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("some-username"),
        active: true,
        sign_in_count: 1,
    };

    //  To get a specific value from a struct, we can use the dot notation. `user1.email`
    println!("{}", user1.email);

    let mut user2 = User {
        email: String::from("user@example.com"),
        username: String::from("other-username"),
        active: true,
        sign_in_count: 2,
    };

    //  If the instance is mutable, we can change the value using the dot notation.
    user2.email = String::from("user2@newexample.com");

    //  Note: The entire instance must be mutable. Rust doesn't allow us to mark only certain fields as mutable.

    let user3 = build_user(String::from("user3@example.com"), String::from("user3"));

    //  The `..` syntax specifies that the remaining fields not not explicitly set should have the same value as the fields in the given instance.
    //  The ..user3 must come last to specify that any remaining fields should get their values from the corresponding fields in user4,
    //  but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.
    let user4 = User {
        email: String::from("user3Alt@example.com"),
        ..user3
    };
    //  Note that the struct update syntax is like assignment with = because it moves the data. In this case user3 is no longer available
    // println!("{}", user3.username);  This will throw an error as user3's username was moved into user4
    //  If we had given user4 new username and email, then active and sign_in_count could have been copied (as they are simple types and implement Copy trait.)
    //  As nothing has moved in such a case, user3 would still be valid.

    tuple_structs()
}

//  We can construct a new instance of the struct using a (constructor) function.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

//  Tuple structs are a special kind of structs that have the added meaning the struct provides, but don't have name associated with their fields
//  rather they just have the type of the fields. Tuple structs are useful when you want ot give the whole tuple a name and make tuple to be different type
//  from other tuples, and naming each field as in a regular struct would be verbose and redundant.

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//  You can also define structs that don't have any fields! These are called unit-like structs because they behave similarly to ()
//  These can be useful in situations in which you need to implement a trait on some type but don't have any data that you want to store in the type itself.
struct AlwaysEqual;

#[allow(unused_variables)]
fn tuple_structs() {
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    let origin = Point(0, 0, 0);

    //  Note black and origin are different types, because they're instances of tuple structs even if they have the same shape.
    //  a function that takes a parameter type of Color, cannot take a Point, even though both are 3 length tuples of i32

    //  Unit-like struct
    let subject = AlwaysEqual;

    example_program();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn example_program() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}", rect);
    println!("The area of the rectangle is {}", area(&rect));

    methods();
}

//  Methods are similar to functions, they're declared with the fn keyword and their name, they can have parameters and return values,
//  and they contain some code that is run when they're called from somewhere else.
//  However, methods are defined in the context of a struct (or enum or trait object), and their first parameter is always self, which represents
//  the instance of the struct the method is being called upon.

//  To define the function within the context of Rectangle, we start with an implementation block (impl). Everything within this block will be
//  associated with Rectangle type. We define a function area that takes a reference to self as its parameter. Methods must have a parameter named &self
//  of type Self for their first parameter. This helps with organization.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //  We can choose to give a method the same name as one of the struct's fields. When we call width followed by parenthesis, Rust knows
    //  we mean the method and not the property. If we don't use parenthesis, Rust knows we mean the field.
    fn width(&self) -> bool {
        self.width > 0
    }
    //  Often, but not always, methods with the same name as a field will be defined to only return the value in the field and do nothing else.
    //  Methods like this are called getters and Rust does not implement them automatically for struct fields as some other languages do.
    //  Getters are useful because you can make the field private and only expose the method as public and thus enable readonly access to that field.

    fn can_hold(&self, r: &Rectangle) -> bool {
        self.width > r.width && self.height > r.height
    }

    //  All functions in the impl block are called associated functions because they're associated with the type named after the impl.
    //  We can define associated functions that don't have self as their first parameter (and thus are not methods) e.g String::from
    //  Associated functions that are not methods are often used for constructors that will return a new instance of the struct.

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    //  Each struct is allowed to have multiple impl blocks.
}

fn methods() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:?}", rect1);
    println!("The area of the rectangle is {}", rect1.area());
    println!("The width exists: {}", rect1.width());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    //  Calling an associated function
    let square = Rectangle::square(15);
    println!("The area of the square is {}", square.area());
}
