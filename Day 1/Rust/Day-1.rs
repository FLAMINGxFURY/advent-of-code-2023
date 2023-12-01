fn main() {
    // Read in the input file and split it into a vector of strings by line
    let input = std::fs::read_to_string("input.txt").unwrap();

    // List to hold all numbers per line
    let mut numbers: Vec<i32> = Vec::new();

    // List of number characters
    let number_chars: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    // List of number words
    let number_words: Vec<&str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    // Iterate over each line
    for line in input.lines() {
        // Create empty dictionaries that hold {position: value} for both number characters and number words
        let mut char_dict: std::collections::HashMap<usize, char> = std::collections::HashMap::new();
        let mut word_dict: std::collections::HashMap<usize, String> = std::collections::HashMap::new();

        populate_dictionaries(line, &number_chars, &number_words, &mut char_dict, &mut word_dict);
        
        numbers.push(extract_cipher(&number_chars, &number_words, &char_dict, &word_dict));
    }

    println!("The sum of all numbers is: {}", numbers.iter().sum::<i32>());
}

fn populate_dictionaries(line: &str, number_chars: &[char], number_words: &[&str], char_dict: &mut std::collections::HashMap<usize, char>, word_dict: &mut std::collections::HashMap<usize, String>) {
    // Iterate over each character in the line
    for (index, character) in line.chars().enumerate() {
        // If the character is a number character, add it to the character dictionary
        if number_chars.contains(&character) {
            char_dict.insert(index, character);
        }

        // If the character is a number word, add it to the word dictionary. We must handle cases for all word sizes
        // and also avoid indexing out of bounds
        else if (index + 3 <= line.len()) && number_words.contains(&&line[index..index + 3]) {
            word_dict.insert(index, line[index..index + 3].to_string());
        }
        else if (index + 4 <= line.len()) && number_words.contains(&&line[index..index + 4]) {
            word_dict.insert(index, line[index..index + 4].to_string());
        }
        else if (index + 5 <= line.len()) && number_words.contains(&&line[index..index + 5]) {
            word_dict.insert(index, line[index..index + 5].to_string());
        }
    }
}

fn extract_cipher(number_chars: &[char], number_words: &[&str], char_dict: &std::collections::HashMap<usize, char>, word_dict: &std::collections::HashMap<usize, String>) -> i32 {
    let mut first_digit = -1; 
    let mut last_digit  = -1;

    // If word_dict is empty, then we know that the line only contains numbers.
    if word_dict.is_empty() {
        // Find the lowest key in char_dict and set first_digit to the value at that key. 
        let first_key = *char_dict.keys().min().unwrap();            
        // using number_chars, find the index of the first_digit in the number_chars list
        first_digit = number_chars.iter().position(|&item| item == char_dict[&first_key]).unwrap() as i32;

        // Find the highest key in char_dict and set last_digit to the value at that key.
        let last_key = *char_dict.keys().max().unwrap();
        // using number_chars, find the index of the last_digit in the number_chars list
        last_digit = number_chars.iter().position(|&item| item == char_dict[&last_key]).unwrap() as i32;
    }

    // If char_dict is empty, then we know that the line only contains words.
    else if char_dict.is_empty() {
        // Find the lowest key in word_dict and set first_digit to the value at that key. 
        let first_key = *word_dict.keys().min().unwrap();            
        // using number_words, find the index of the first_digit in the number_words list
        first_digit = number_words.iter().position(|&item| item == word_dict[&first_key].as_str()).unwrap() as i32;

        // Find the highest key in word_dict and set last_digit to the value at that key.
        let last_key = *word_dict.keys().max().unwrap();
        // using number_words, find the index of the last_digit in the number_words list
        last_digit = number_words.iter().position(|&item| item == word_dict[&last_key].as_str()).unwrap() as i32;
    }

    // Else, determine the first and last positions from both dictionaries
    else {
        // Find the lowest key in char_dict and set first_digit to the value at that key. 
        let first_char_key = *char_dict.keys().min().unwrap();
        let first_word_key = *word_dict.keys().min().unwrap();
        let first_char_digit = number_chars.iter().position(|&item| item == char_dict[&first_char_key]).unwrap() as i32;
        let first_word_digit = number_words.iter().position(|&item| item == word_dict[&first_word_key].as_str()).unwrap() as i32;
        
        if first_char_key < first_word_key {
            first_digit = first_char_digit;
        }
        else {
            first_digit = first_word_digit;
        }

        // Find the highest key in char_dict and set last_digit to the value at that key.
        let last_char_key = *char_dict.keys().max().unwrap();
        let last_word_key = *word_dict.keys().max().unwrap();
        let last_char_digit = number_chars.iter().position(|&item| item == char_dict[&last_char_key]).unwrap() as i32;
        let last_word_digit = number_words.iter().position(|&item| item == word_dict[&last_word_key].as_str()).unwrap() as i32;

        if last_char_key > last_word_key {
            last_digit = last_char_digit;
        }
        else {
            last_digit = last_word_digit;
        }
    }

    return first_digit * 10 + last_digit;
}