// # 10 Modules

// Rust provides a powerful module system that can be used to hierarchically split code into logical units (modules), and manage visibility (public/private) between them.

// A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

fn main() {
    showcase_visibility();
    showcase_struct_visibility();
    showcase_use();
    showcase_super_and_self();
}

// ## 10.1 Visibility

// By default, the items in a module have private visibility, meaning they can only be accessed from within the same module.
// To make an item public, you can use the `pub` keyword. This allows the item to be accessed from outside the module.

// A module named `my_mod`
mod my_mod {
    // Items in modules default to private visibility
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` keyword to override default visibility
    pub fn function() {
        println!("called `my_mod::public_function()`");
    }

    // ITems can access other items in the same module, even if they are private
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n>");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible within the given path. `path` must be an parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private paraent items will still restrict the visibility of a child even if it is declared as visible within a bigger scope
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn showcase_visibility() {
    // Modules allow disambiguation between items that have the same name
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be accessed from outside the parent module
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified in the path
    // Error! function `public_function_in_my_mod` is private
    // my_mod::nested::public_function_in_my_mod();

    // Private items of a mdoule cannot be directly accessed, even if nested in a public module

    // Error! `private_function` is private
    // my_mod::private_function();

    // Error! `private_function` is private
    // my_mod::nested::private_function();

    // Error! `private_nested` is a private module
    // my_mod::private_nested::function();

    // Error! `private_nested` is a private module
    // my_mod::private_nested::restricted_function();
}

// ## 10.2 Struct Visibility

// Structs have an extra level of visibility with their fields. The visibility defaults to private, and can be overridden with the `pub` modifier.
// The visibility only matters when a struct is accessed from outside the module where it is defined, and has the goal of hiding information (encapsulation)

mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct ClosedBox<T> {
        #[allow(dead_code)]
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method that returns a new instance of `ClosedBox`
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

fn showcase_struct_visibility() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox {
        contents: "public information",
    };

    // and their fields can be accessed as usual
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names, because the fields are not visible
    // Error! `ClosedBox` has private fields
    // let closed_box = my::ClosedBox {
    //     contents: "classified information",
    // };

    // However, structs with private fields can be created using public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed directly
    // Error! `contents` is private
    // println!("The closed box contains: {}", _closed_box.contents);
}

// ## 10.3 Use Declaration

// The `use` declaration can be used to bind a full path to a new name, for easier acccess.

// ```rs
// use crate::my_mod::nested::function as nested_function;
//
// fn main() {
//     nested_function();
// }
// ```

// You can also use the `as` keyword to bind imports to a different name:

#[allow(dead_code)]
fn func() {
    println!("called `func()`");
}

mod deeply {
    pub mod nested {
        pub fn func() {
            println!("called `deeply::nested::func()`");
        }
    }
}

// Bind the `deeply::nested::function` path to `my_func`
use deeply::nested::func as my_func;

fn showcase_use() {
    // Easier access to `deeply::nested::func()` through the `my_func` alias
    my_func();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::func as my_func;`
        // This `my_func` shadows the outer `my_func`
        use deeply::nested::func as my_func;

        // `use` bindings have a local scope. In this case, the shadowing of `my_func` is only in this block
        my_func();

        println!("Leaving block");
    }

    // The original `my_func` is not shadowed outside the block
    my_func();
}

// ## 10.4 `super` and `self`

//  The `super` and `self` keywords can be used in the path to remove ambiguity when accessing items and to prevent unnecessary hardcoding of paths.

mod super_and_self {
    fn function() {
        println!("called function()");
    }

    mod cool {
        pub fn function() {
            println!("called `cool::function()`");
        }
    }

    pub mod my {
        fn function() {
            println!("called `my::function()`");
        }

        mod cool {
            pub fn function() {
                println!("called `my::cool::function()`");
            }
        }

        pub fn indirect_call() {
            // Let's access all the functions named `function` from this scope!
            println!("called `my::indirect_call()`, that\n> ");

            // The `self` keyword  refers to the current module scope - in this case `my`
            // Calling `self::function()` and calling `function()` directly both give same result, as they referer to the same function.
            self::function();
            function();

            // We can also use `self` to access another module inside `my`:
            self::cool::function();

            // The `super` keyword refers to the parent module scope - in this case, the outer `super_and_self` module
            super::function();

            // This will bind to the `cool::function()` in the `super_and_self` module, not the one in `my
            {
                use crate::super_and_self::cool::function as cool_function;
                cool_function();
            }
        }
    }
}

fn showcase_super_and_self() {
    super_and_self::my::indirect_call();
}

// ## 10.5 File Hierarchy

// Modules can be mapped to a file/directory hierarchy.
// Files with the same name as the module and a `.rs` extension are used for modules, and directories with a `mod.rs` file are used for nested modules.
