//  This brings the std::io (standard input/output) and std::cmp::Ordering libraries into scope
use std::cmp::Ordering;
use std::io;

//  Use rand crate
use rand::Rng;

//  The main function is the entry point to the program
fn main() {
    println!("Guess the number!");

    //  Generate a random number between 0 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //  Infinitely loop till the user guesses the correct number (or rage quits)
    loop {
        //  Prompt the user for the input
        println!("Please input your guess:");

        //  Initialize a variable to hold user's input
        //  Variables are immutable by default. i.e. their value cannot be reassigned after binding
        //  They must be explicitly made mutable using the `mut` keyword
        //  `String` is a string type provided by the standard library that is a growable UTF8 encoded bit of text
        //  The `::` syntax indicates that `new` is an associated function of the `String` type
        let mut guess = String::new();

        //  Get the user's input from stdin
        io::stdin()
            //  Reads std input appends it to guess
            //  & indicates this argument is a reference and not a copy.
            //  Like variables, references are also immutable by default. Hence the `&mut guess`. More on all this later.
            .read_line(&mut guess)
            //  The `read_line` method can fail, so we provide an error message if it does
            .expect("Failed to read input");

        //  The `{}` is a placeholder that will take the value of guess
        println!("You guessed {}", guess);

        //  Parse guess as number. Using let guess again allows us to shadow the previous guess with this one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //  Compare guess with the secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                //  If equal, then congratulate and break loop
                println!("You Win! 🎉");
                //  Break the loop and exit
                break;
            }
        }
    }
}
