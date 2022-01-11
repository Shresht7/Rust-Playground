fn main() {
    display_title("Temperature Converter");
    let (tempt, unit) = prompt_user_for_temperature("Input temperature value: ");
    let res = convert_temperature(tempt, unit);
    println!("The converted temperature is: {}", res)
}

//  Display Program Title
fn display_title(title: &str) {
    let str_len = title.len(); //  Get the string length of the title

    let decorator = "-".repeat(str_len); //  Create decorator lines

    //  Display Title
    println!("{}", decorator);
    println!("{}", title);
    println!("{}", decorator);
}

//  Prompts the user for an input and returns the response
fn prompt_user_for_temperature(pmt: &str) -> (f32, char) {
    println!("{}", pmt); //  Show prompt

    let mut res = String::new();

    //  Read user response
    std::io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");

    let unit = res.remove(res.len() - 3);
    res = res.chars().take(res.len() - 2).collect();

    //  Convert response string to a i32 number
    let result = res.trim().parse::<f32>().expect("Not a valid number");

    //  Return the response
    (result, unit)
}

//  Temperature Converter
fn convert_temperature(temp: f32, unit: char) -> f32 {
    if unit == 'F' {
        //  Fahrenheit to Celsius
        (5.0 / 9.0) * (temp - 32.0)
    } else if unit == 'C' {
        //  Celsius to Fahrenheit
        (9.0 / 5.0) * temp + 32.0
    } else {
        return -1024.0; //  Invalid condition
    }
}
