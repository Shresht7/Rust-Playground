// # Query Strings

// Query strings are URL-encoded forms that apepar in the URL of a request.
// Query parameters are declared like path parameters but otherwise handled like regular form fields.
// Because, dynamic parameters are form types, they can be single values, collections, nested collections, or anything in between, just like any other form

#[macro_use]
extern crate rocket;

// ## Static Parameters
// A request matches a route iff its query string contains all the static parameters in the router's query sting. A route with a static parameter `param`
// (any UTF-8 text string) in a query wqill only match requests with that exact path segment in its query string.

// Only the static parameters in a query route string affect routing. Dynamic parameters are allowed to be missing by default

// For example, the route below will match requests with path `/` and at least the query segments `hello` and `cat=meow`
#[get("/?hello&cat=meow")]
fn cats() -> &'static str {
    "Meow!"
}

// ## Dynamic Parameters

// A single dynamic parameter of `<param>` acts identically to a form field declared as `param`.

#[derive(Debug, PartialEq, FromFormField)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, PartialEq, FromForm)]
struct Pet<'r> {
    name: &'r str,
    age: usize,
}

#[derive(Debug, PartialEq, FromForm)]
struct Person<'r> {
    name: &'r str,
    age: usize,
    pet: Pet<'r>,
    favorite_color: Color,
}

#[get("/?<name>&<color>&<person>&<other>")]
fn hello(name: &str, color: Vec<Color>, person: Person<'_>, other: Option<usize>) -> String {
    let mut res = String::new();
    res.push_str(&format!("Hello, {}!\n", name));
    res.push_str(&format!("Your favorite colors are: {:?}\n", color));
    res.push_str(&format!(
        "Your pet's name is {}, and they are {} years old.\n",
        person.pet.name, person.pet.age
    ));
    if let Some(other) = other {
        res.push_str(&format!("Other parameter: {}\n", other));
    }
    res
}

// Note that, like forms, parsing is field-ordering insensitive and lenient by default

// ## Trailing Parameters

// A trailing dynamic parameter of `<param..>` collects all of the query segments that don't otherwise match a declared static or dynamic parameter.

#[derive(Debug, FromForm)]
struct User<'r> {
    name: &'r str,
    active: bool,
}

#[get("/?hello&<id>&<user..>")]
fn user(id: usize, user: User<'_>) -> String {
    if !user.active {
        return format!(
            "Hello, {}! Your id is {}, but your account is inactive.",
            user.name, id
        );
    }
    format!(
        "Hello, {}! Your id is {}, and your user data is: {:?}",
        user.name, id, user
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![cats, hello, user])
}
