use std::{collections, ptr::read};

use text_io::read;

fn string_to_int() -> u32 {
    println!("Welcome to the String to integer to Numerology! \n Please enter a word:");

    //We get the user input
    let user_string: String = read!();

    //Takes ownership of user_string
    let char_vec: Vec<char> = user_string.chars().collect();

    //We define num_vec as a vector which will be used later
    let mut num_vec: Vec<u32> = vec![];

    //We iterate over char_vec while converting each character to its corresponding number and pushing it into num_vec
    for ch in char_vec.iter().enumerate() {
        let char_to_int = char_vec[ch.0].to_ascii_uppercase() as u32 + 1 - 'A' as u32;

        num_vec.push(char_to_int);
    }

    //We print the sum of the numbers in num_vec
    let _sum: u32 = num_vec.iter().sum();
    println!(
        "The sum of the position of the letters of your word is:{}",
        _sum
    );

    return _sum;
}

/* fn int_to_string() {
    println!("Welcome to the Integer to String Converter! \n Please enter a word:");

    //We get the user input
    let user_string: u32 = read!();

    //Takes ownership of user_string
    let char_vec: Vec<char> = user_string.chars().collect();

    //We define num_vec as a vector which will be used later
    let mut num_vec: Vec<u32> = vec![];

    //We iterate over char_vec while converting each character to its corresponding number and pushing it into num_vec
    for ch in char_vec.iter().enumerate() {
        let char_to_int = char_vec[ch.0].to_ascii_uppercase() as u32 + 1 - 'A' as u32;

        num_vec.push(char_to_int);
    }

    //We print the sum of the numbers in num_vec
    let _sum: u32 = num_vec.iter().sum();
    println!(
        "The sum of the position of the letters of your word is:{}",
        _sum
    );

    todo!()
} */

fn string_to_int_leet() -> i32 {
    let s: String = read!();

    let s = s.trim_start().to_string();

    println!("s: {}", s);

    let mut slice_str = s.clone();

    if s.trim_start().starts_with("-") {
        slice_str = s[1..].to_string();
    }

    if s.trim_start().starts_with("+") {
        slice_str = s[1..].to_string();
    }

    if slice_str.is_empty() {
        println!("Is empty");
    }
    println!("{}", slice_str);

    let mut trimmed_s: String = slice_str
        .trim_start()
        .chars()
        .take_while(|slice_str| slice_str.is_digit(10))
        .collect();

    if trimmed_s.is_empty() {
        println!("Is empty")
    }

    if s.starts_with("-") {
        trimmed_s.insert(0, '-');
    }

    match trimmed_s.parse::<i32>() {
        Ok(number) => number,
        Err(message) => match message.kind() {
            std::num::IntErrorKind::PosOverflow => i32::MAX,
            std::num::IntErrorKind::NegOverflow => i32::MIN,
            _ => 0,
        },
    }
}

fn main() {
    println!(
        "Welcome to the Numerology program! \n Input either 1, for the Ceasar's Cypher and 2 for the Char - Int converter"
    );

    let option: i32 = read!();

    match option {
        1 => {
            ceasar_cipher();
        }
        2 => {
            string_to_int();
        }
        3 => {
            string_to_int_leet();
        }
        _ => {
            println!("Invalid option");
        }
    }

    fn ceasar_cipher() -> String {
        println!("Welcome to Ceasar's Cypher! \n Input the word / sentence you want to convert? ");

        let user_string: String = read!();

        let char_vec: Vec<char> = user_string.chars().collect();

        let mut num_vec: Vec<u32> = vec![];

        for ch in char_vec.iter().enumerate() {
            let char_to_int = char_vec[ch.0].to_ascii_uppercase() as u32 + 4 - 'A' as u32;

            num_vec.push(char_to_int);
        }

        // Example vector with ASCII values
        let char_vec2: Vec<char> = num_vec
            .iter()
            .map(|&num| (num as u8 + 'A' as u8 - 1) as char)
            .collect();

        let cipher = char_vec2.iter().collect::<String>();

        println!("The word ciphered is: {}", cipher);

        return cipher;
    }
    fn fibonacci() -> i32 {
        let user_input: i32 = read!();

        let a = 0;
        let b = 1;
        let mut c = a + b;

        for i in 0..user_input {}

        0
    }
}
