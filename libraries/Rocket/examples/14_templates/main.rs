// # Templates

// Rocket has first class templating support that works largely through a `Template` responder in the `rocket_dyn_templates` contrib library.

#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    let context = /* Object-like value */ ();

    // Templates are rendered with the `render` method.The method takes in the name of a template and a context to render the template with.
    // The context can be of any type that implements the `Serialize` trait.
    Template::render("index", context)
}

// You can also use the `context!` macro to create ad-hoc templating contexts, without defining a new type.

#[get("/context")]
fn context() -> Template {
    Template::render(
        "index",
        context! {
            foo: 123
        },
    )
}

// To render a template, it must be registered first. The `Template` faring automatically registers all discoverable templates when attached.

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, context])
}

// Rocket discovers templates in the configurable `template_dir` directory. Templating support in Rocket is engine agnostic.
// The engine used to render a template depends on the template's file extension. For example, if a file ends with `.hbs`, Handlebars is used,
// while if a file ends with `.tera` then Tera is used.

// Live Reloading: When running in development mode, Rocket will automatically reload templates when they are changed on disk.
