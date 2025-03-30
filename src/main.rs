use text_io::read;

fn numerology() {
    println!("Welcome to the Word to Numerology! \n Please enter a word:");
    /* let letters = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let numbers = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26,
    ];*/

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
    println!("{}", _sum);
}

fn main() {}

fn ceasar_cyper() {
    println!("Welcome to Ceasar's Cypher! \n Input the word / sentence you want to convert? ");

    let user_string: String = read!();

    let char_vec: Vec<char> = user_string.chars().collect();

    let mut num_vec: Vec<u32> = vec![];

    for ch in char_vec.iter().enumerate() {
        let char_to_int = char_vec[ch.0].to_ascii_uppercase() as u32 + 1 - 'A' as u32;

        num_vec.push(char_to_int);
    }
}
