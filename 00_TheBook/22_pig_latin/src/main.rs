const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let s = "The first consonant of each word is moved to the end of the word and -ay is added. Words that start with a vowel have -hay added to the end.";
    let mut res = String::new();
    let words = s.split(" ");
    for word in words {
        if let Some(first_char) = word.to_ascii_lowercase().chars().nth(0) {
            if VOWELS.contains(&first_char) {
                res.push_str(word);
                res.push_str("-hay");
            } else {
                let mut letters = word.chars().skip(1);
                while let Some(ch) = letters.next() {
                    res.push(ch);
                }
                res.push('-');
                res.push(first_char);
                res.push_str("ay");
            }
        }
        res.push(' ');
    }
    println!("{res}");
}
