// # Supertraits

// Rust doesn't have "inheritance", but you can define a trait as being a superset of another trait.

trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Studen
// Implementing Student requires you to also implement Person
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// ComSciStudent is a subtrait of both Programmer and Student. Implementing ComSciStudent requries you to implement both Programmer and Student
trait ComSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn ComSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git Username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl Student for CSStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Person for CSStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl ComSciStudent for CSStudent {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn main() {
    let student = CSStudent {
        name: "Alice".to_string(),
        university: "MIT".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "alice123".to_string(),
    };

    println!("{}", comp_sci_student_greeting(&student));
}
