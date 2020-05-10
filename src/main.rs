use std::io;

fn main() {
    println!("Please input word for pig latin");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.truncate(input.trim().len());

    println!("Pig latinized: {}", pig_latinize(&mut input));
}

fn pig_latinize(input: &mut String) -> &mut String {
    let first_letter = input.chars().next().unwrap();

    if is_vowel(first_letter) {
        input.push_str("hay");

        input
    } else {
        input.remove(0);
        input.push(first_letter);
        input.push_str("ay");

        input
    }
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
