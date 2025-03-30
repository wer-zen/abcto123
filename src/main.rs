use text_io::read;

fn numerology() {
    println!("Welcome to the Word to Numerology! \n Please enter a word:");

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
}

fn main() {
    println!(
        "Welcome to the Numerology program! \n Input either 1, for the Ceasar's Cypher and 2 for the Char - Int converter"
    );

    let option: String = read!();

    match option.as_str() {
        "1" => ceasar_cyper(),
        "2" => numerology(),
        _ => println!("Invalid option"),
    }
}

fn ceasar_cyper() {
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
}
