// # Forms

// Forms are one of the most common types of data handled in web applications.
// Rocket supports both `multipart` and `x-www-form-urlencoded` forms out of the box, enabled by the `Form` data guard and derivable from `FromForm` trait.

#[macro_use]
extern crate rocket;

use rocket::form::{Form, FromForm};

#[derive(FromForm)]
struct Task<'r> {
    completed: bool,
    r#type: &'r str,
}

#[post("/task", data = "<task>")]
fn create_task(task: Form<Task<'_>>) -> String {
    format!(
        "Created task: {} (completed: {})",
        task.r#type, task.completed
    )
}

// Form is data guard as long as its generic parameter implements the `FromForm` trait.

// ## Multipart Forms

// Multipart forms are handled transparently, with no additional effort. Most `FromForm` types can parse themselvess from incoming data stream.

use rocket::fs::TempFile;

#[derive(FromForm)]
struct Upload<'r> {
    save: bool,
    file: TempFile<'r>,
}

#[post("/upload", data = "<upload>")]
fn upload_form(upload: Form<Upload<'_>>) -> String {
    // In a real application, you would persist the file here.
    // For this example, we'll just return a dummy response.
    if upload.save {
        format!("Save: {}", upload.file.name().unwrap_or("file"))
    } else {
        format!("Not Save {}", upload.file.name().unwrap_or("file"))
    }
}

// ## Parsing Strategy

// Rocket's `FromForm` parsing is lenient by default: a `Form<T>` will parse successfully from an incoming form even if it contains
// extra, duplicate, or missing fields. Extras or duplicates are simply ignored -- no validation or parsing of the fields occur -- and missing fields
// are filled with defaults when available.
// To make the form validation strict use the `Form<Strict<T>>` data type, which emits errors if there are any extra or missing fields, irrespective of defaults.

use rocket::form::Strict;

#[post("/todo", data = "<task>")]
fn new_task(task: Form<Strict<Task<'_>>>) -> String {
    format!(
        "Created task: {} (completed: {})",
        task.r#type, task.completed
    )
}

// `Strict` can also be used to make individual fields strict while keeping the overal structure and remaining fields lenient:

#[derive(FromForm)]
struct Input {
    required: Strict<bool>,
    uses_default: bool,
}

#[post("/input", data = "<input>")]
fn new_input(input: Form<Input>) -> String {
    let input = input.into_inner();
    let required_value = input.required.into_inner();
    let uses_default = input.uses_default;
    format!(
        "Received input: required = {}, uses_default = {}",
        required_value, uses_default
    )
}

// `Lenient` is the _lenient_ analog to `Strict`, which forces parsing to be lenient. `Form` is lenient by default, so `Form<Lenient<T>>` is redundant,
// but `Lenient` can be used to overwrite a strict parse as lenient: `Option<Lenient<T>>`

// ## Defaults
// A form guard may specify a default value to use when a field is missing. The default value is used only when parsing is lenient.
// When strict, all errors, including missing fields, are propagated directly.
// Some types with defaults include `bool`, which defaults to `false`, useful for checkboxes, `Option<T>`, which defaults to `None`, and
// `form::Result`, which defaults to `Err(Missing)` or otherwise collects errors in an `Err` of `Errors<'_>`.
// Default guards can be used just like any other form guard:

use rocket::form;

#[derive(FromForm)]
struct MyForm<'v> {
    maybe_string: Option<&'v str>,
    ok_or_error: form::Result<'v, Vec<&'v str>>,
    here_or_false: bool,
}

#[post("/form/basic", data = "<form>")]
fn form_basic(form: Form<MyForm<'_>>) -> String {
    let form = form.into_inner();
    format!(
        "maybe_string: {:?}, ok_or_error: {:?}, here_or_false: {}",
        form.maybe_string, form.ok_or_error, form.here_or_false
    )
}

// The default can be overriden or unset using the `#[field(default expr)]` field attribute. If `expr` is not literally `None`,
// the parameter sets the default value of the field to be `expr.into()`. If `expr` is `None`, the parameter unsets the default value of the field, if any
#[derive(FromForm)]
struct MyFormWithDefaults {
    // Set the default value to be `"hello"`
    //
    // Note how an `&str` is automatically converted into a `String`
    #[field(default = "hello")]
    greeting: String,

    // Remove the default of value of `false`, requiring all parses of `MyFormWithDefaults`
    // to contain an `is_friendly` field.
    #[field(default = None)]
    is_friendly: bool,
}

#[post("/form/defaults", data = "<form>")]
fn form_defaults(form: Form<MyFormWithDefaults>) -> String {
    let form = form.into_inner();
    format!(
        "greeting: {}, is_friendly: {}",
        form.greeting, form.is_friendly
    )
}

// ## Field Renaming
// By default, Rocket matches the name of an incoming form field to the name of a struct field.
// This can be overridden using the `#[field(name = "name`)]` or `#[field(name = uncased("name")]` field annotation.
// The `uncased` variant case-insensitively matches field names.
#[derive(FromForm)]
struct ExternalServiceOne<'r> {
    #[field(name = "first-Name")]
    first_name: &'r str,
}

#[post("/form/rename/1", data = "<form>")]
fn form_rename_one(form: Form<ExternalServiceOne<'_>>) -> String {
    let form = form.into_inner();
    format!("first_name: {}", form.first_name)
}

// If you want to accept both `firstName` case-insensitively as well as `first_name` case-insensitively, you can use multiple annotations:

#[derive(FromForm)]
struct ExternalServiceTwo<'r> {
    #[field(name = uncased("firstName"))]
    #[field(name = "first-Name")]
    first_name: &'r str,
}

#[post("/form/rename/2", data = "<form>")]
fn form_rename_two(form: Form<ExternalServiceTwo<'_>>) -> String {
    let form = form.into_inner();
    format!("first_name: {}", form.first_name)
}

// If you wanted any of `first-name`, `first_name` or `firstName`, in each instance case-insensitively, you would write:

#[derive(FromForm)]
struct ExternalServiceThree<'r> {
    #[field(name = uncased("first-name"))]
    #[field(name = uncased("first_name"))]
    #[field(name = uncased("firstName"))]
    first_name: &'r str,
}

#[post("/form/rename/3", data = "<form>")]
fn form_rename_three(form: Form<ExternalServiceThree<'_>>) -> String {
    let form = form.into_inner();
    format!("first_name: {}", form.first_name)
}

// ## Ad-Hoc Validation

// Fields of forms can be easily ad-hoc validated via the `#[field(validate)]` attribute.
// As an example, consider a form field `age: u16`, which we'd like to ensure is greater than `21`. The following structure accomplishes this:

#[derive(FromForm)]
struct Person {
    #[field(validate = range(21..))]
    age: u16,
}

#[post("/validate/adhoc-person", data = "<person>")]
fn validate_adhoc_person(person: Form<Person>) -> String {
    let person = person.into_inner();
    format!("age: {}", person.age)
}

// The expression `range(21..` is a call to `form::validate::range`. Any function in the `form::validate` module can be called and other fields can be passed in by using `self.$field`
// where `$field` is the name of the field in the structure definition. You can also apply more than one validation field by using multiple attributes.

#[derive(FromForm)]
struct Password<'r> {
    #[field(name = "password")]
    value: &'r str,

    #[field(validate = eq(self.value))]
    #[field(validate = omits("no"))]
    _confirm: &'r str,
}

#[post("/validate/adhoc-password", data = "<password>")]
fn validate_adhoc_password(password: Form<Password<'_>>) -> String {
    let password = password.into_inner();
    format!("password: {}", password.value)
}
// In reality, the expression after `validate = ` can be any expression as long as it evaluates to a value of type `Result<(), Errors<'_>>` (aliases by `form::Result`),
// where an `Ok` means that the validation was successful and `Err` indicates that error(s) occurred.

// For instance, if you wanted to implement an ad-hoc Luhn validator for credit-card like numbers, you might write:
use rocket::form::Error;
use rocket::time::Date;

#[derive(FromForm)]
#[warn(dead_code)]
struct CreditCard {
    #[field(validate = luhn(self._cvv, &self._expiration))]
    number: u64,

    #[field(validate = range(..9999))]
    _cvv: u16,

    _expiration: Date,
}

fn luhn<'v>(number: &u64, _cvv: u16, _exp: &Date) -> form::Result<'v, ()> {
    // In a real application, you would implement the Luhn algorithm here.
    // For this example, we'll just return an error if the number is not 1234567890123456.
    if *number == 1234567890123456 {
        Ok(())
    } else {
        Err(Error::validation("invalid credit card number").into())
    }
}

#[post("/validate/adhoc-credit-card", data = "<card>")]
fn validate_adhoc_credit_card(card: Form<CreditCard>) -> String {
    let card = card.into_inner();
    format!("number: {}", card.number)
}

// ## Wrapping Validators

// If a particular validation is applied in more than one place, prefer creating a type that encapsulates and respresents the validated value.
// For example, if your application often validates `age` fields, consider creating a custom `Age` form guard that always applies the validation:

#[derive(FromForm)]
#[field(validate = range(18..150))]
struct Age(u16);

#[post("/validate/age", data = "<age>")]
fn validate_age(age: Form<Age>) -> String {
    let age = age.into_inner();
    format!("age: {}", age.0)
}

// ## Collections

// ??? https://rocket.rs/guide/v0.5/requests/#collections

// ## Nesting
// Forms can be nested
#[derive(FromForm)]
struct NestedMyForm<'r> {
    owner: NestedPerson<'r>,
    pet: NestedPet<'r>,
}

#[derive(FromForm)]
struct NestedPerson<'r> {
    name: &'r str,
}

#[derive(FromForm)]
struct NestedPet<'r> {
    name: &'r str,
    #[field(validate = eq(true))]
    good_pet: bool,
}

// To parse into a `NestedMyForm`, a form with the following fields must be submitted:
// - `owner.name` - string
// - `pet.name` - string
// - `pet.good_pet` - boolean

// Such a form, URL-encoded, may look like `owner.name=Bob&pet.name=Rover&pet.good_pet=true`
// All oof these are identical to the previous
// - `owner[name]=Bob&pet[name]=Rover&pet[good_pet]=true`
// - `owner.name=Bob&pet[name]=Rover&pet.good_pet=true`

#[post("/form/nested", data = "<form>")]
fn form_nested(form: Form<NestedMyForm<'_>>) -> String {
    let form = form.into_inner();
    format!(
        "owner.name: {}, pet.name: {}, pet.good_pet: {}",
        form.owner.name, form.pet.name, form.pet.good_pet
    )
}

// ## Vectors

// A form can also contain sequences (vectors of values).
#[derive(FromForm)]
struct VectorForm {
    numbers: Vec<usize>,
}

#[post("/form/vector", data = "<form>")]
fn form_vector(form: Form<VectorForm>) -> String {
    let form = form.into_inner();
    format!("numbers: {:?}", form.numbers)
}

// To parse into a `VectorForm`, `numbers[$k]` must be submitted (or equivalently, numbers.$k),
// where `$key` is the key used to determine whether to push the rest of the field to the last element in the vector or create a new.
// if the key is the same as the previously seen key by the vector, then the field's value is pushed to the last element in the vector.
// Otherwise, a new element is created.The actual value of $k is irrelevant, it is only used for comparison, has no semantic meaning, and is not remembered by `Vec`.

// ### Nesting in Vectors
// ### Nested Vectors

// ## Maps

// A form can also contain maps
use std::collections::HashMap;

#[derive(FromForm)]
struct MapForm {
    ids: HashMap<String, usize>,
}

#[post("/form/map", data = "<form>")]
fn form_map(form: Form<MapForm>) -> String {
    let form = form.into_inner();
    format!("ids: {:?}", form.ids)
}

// To parse into a `MapForm`, a form with the following fields must be submitted:
// - `$ids[$string]` - usize(or equivalently, `id.$string`)
// where `$string` is the "key" used to determine which value in the map to push to rest of the field to.
// Unlike with vectors, the key does have a semantic meaning and is remembered, so ordering of fields is inconsequential:
// a given string `$string` always maps to the same element.

// { "a" => 1, "b" => 2 }

// "ids[a]=1&ids[b]=2"
// "ids[a]=1&ids[a]=2&ids[b]32"
// "ids.a=1&ids.a=2"

// ```rs
// MapForm {
//     ids: map! {
//         "a" => 1usize,
//         "b" => 2usize
//     }
// }
// ```

// Both the key and values of a `HashMap` can be any type that implements `FromForm`.

#[derive(Debug, FromForm)]
struct MapFormWithComplexKeys {
    ids: HashMap<usize, MapPerson>,
}

#[derive(Debug, FromForm)]
struct MapPerson {
    name: String,
    age: usize,
}

#[post("/form/map-complex", data = "<form>")]
fn form_map_complex(form: Form<MapFormWithComplexKeys>) -> String {
    let form = form.into_inner();
    let mut res = String::new();
    for (id, person) in form.ids {
        res.push_str(&format!(
            "id: {}, name: {}, age: {}\n",
            id, person.name, person.age
        ));
    }
    res
}

// To parse into a `MapFormWithComplexKeys`, a form with the following fields must be submitted:
// - ids[$usize].name - string
// - ids[$usize].age - usize

// "ids[0]name=Bob&ids[0]age=3&ids[1]name=Alice&ids[1]age=10"
// "ids[0].name=Alice&ids[1].name=Bob&ids[0].age=3&ids[1].age=10"

// Which parses into the following:
// MapFormWithComplexKeys {
//     ids: map! {
//         0 => MapPerson { name: "Bob".into(), age: 3 },
//         1 => MapPerson { name: "Alice".into(), age: 10 }
//     }
// }
#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            create_task,
            upload_form,
            new_task,
            new_input,
            create_task,
            new_input,
            form_basic,
            form_defaults,
            form_rename_one,
            form_rename_two,
            form_rename_three,
            validate_adhoc_person,
            validate_adhoc_password,
            validate_adhoc_credit_card,
            validate_age,
            form_nested,
            form_vector,
            form_map,
            form_map_complex
        ],
    )
}
