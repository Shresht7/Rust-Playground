use clap::{App, Arg};
use rand::Rng;

fn main() {
    let matches = App::new("Roll Die")
        .version("0.1.0")
        .author("Shresht7")
        .about("Rolling die from command line")
        .arg(
            Arg::new("roll")
                .short('r')
                .long("roll")
                .takes_value(true)
                .help("Rolls die. e.g. 1d20, 3d8"),
        )
        .get_matches();

    let die = matches.value_of("roll").unwrap_or("1d20");

    let result = roll_die(&die);

    println!("The roll is {}", result);
}

// Roll Die
fn roll_die(die: &str) -> u32 {
    //  Initialize RNG
    let mut rng = rand::thread_rng();

    //  Split string and retrieve die number and range
    let mut die_split = die.split("d");
    let number_of_die: u32 = match die_split.next() {
        Some(x) => x.parse::<u32>().expect("Failed to parse as u32"),
        None => 1,
    };

    let range_of_die: u32 = die_split
        .next()
        .expect("failed to retrieve range")
        .parse::<u32>()
        .expect("Failed to parse as u32");

    //  Calculate and return result
    let mut result = 0;
    let mut i = 0;
    while i < number_of_die {
        result = result + rng.gen_range(1..=range_of_die);
        i += 1;
    }
    result
}
