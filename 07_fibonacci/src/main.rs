fn main() {
    let n = prompt("Enter a number: ");
    generate_fibonacci(n);
}

//  Prompt the user and return the response
fn prompt(pmt: &str) -> u32 {
    println!("{}", pmt); //  Prompt message

    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to read user response");

    return response
        .trim()
        .parse::<u32>()
        .expect("Failed to parse the response as a number");
}

//  Generates and displays the fibonacci sequence
fn generate_fibonacci(n: u32) {
    let mut prev: u32 = 0; //  Previous number
    let mut curr: u32 = 1; //  Current number
    let mut next: u32 = 1; //  Next number in the sequence

    println!("The fibonacci sequence is \n1");

    //  Loop until the sequence reaches the specified limit
    while next < n {
        next = curr + prev; //  Get the next number in the sequence
        println!("{}", next);
        prev = curr;
        curr = next;
    }
}
