// # Typed URIs

// Rocket's `uri!` macro allows you to build URIs in a robust, type-safe, and URI-safe manner. Type or route parameter mismatches are caught
// at compile-time, and changes to route URIs are automatically reflected in the generated URIs.

// The `uri!` macro returns an `Origin` structure with the URI of the supplied route interpolated with the given values.
// Each value passed into `uri!` is rendered in its appropriate place in the URI using `UriDisplay` implementations.
// The `UriDisplay` trait implementations ensure that the rendered value is URI-safe.

// ```rs
// // with unnamed parameters, in route path declaration order
// let mike = uri!(person(101, "Mike Smith", Some(28)));
// assert_eq!(mike.to_string(), "/101/Mike%20Smith?age=28");
//
// // with named parameters, order irrelevant
// let mike = uri!(person(name = "Mike", id = 101, age = Some(28)));
// assert_eq!(mike.to_string(), "/101/Mike?age=28");
// let mike = uri!(person(id = 101, age = Some(28), name = "Mike"));
// assert_eq!(mike.to_string(), "/101/Mike?age=28");
//
// // with a specific mount-point
// let mike = uri!("/api", person(id = 101, name = "Mike", age = Some(28)));
// assert_eq!(mike.to_string(), "/api/101/Mike?age=28");
//
// // with optional (defaultable) query parameters ignored
// let mike = uri!(person(101, "Mike", _));
// assert_eq!(mike.to_string(), "/101/Mike");
// let mike = uri!(person(id = 101, name = "Mike", age = _));
// assert_eq!(mike.to_string(), "/101/Mike");
// ```

#[macro_use]
extern crate rocket;

// The `UriDisplay` trait can be derived for custom types.
// For types that appear in the path part of the URI, derive using `UriDisplayPath`
// For types that appear in the query part of the URI, derive using `UriDisplayQuery`

#[derive(Debug, FromForm, UriDisplayQuery)]
struct UserDetails<'r> {
    age: Option<usize>,
    nickname: &'r str,
}

#[post("/user/<id>?<details..>")]
fn add_user(id: usize, details: UserDetails) -> String {
    format!("Added user with id {} and details: {:?}", id, details)
}

// ```rs
// let link = uri!(add_user(120, UserDetails { age: Some(20), nickname: "Bob".into() }));
// assert_eq!(link.to_string(), "/120?age=20&nickname=Bob");
//```

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![add_user])
}
