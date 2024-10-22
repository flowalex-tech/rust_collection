use std::collections::HashSet;
use std::io;

fn main() {
    let mut word_in = String::new();
    println!("Please enter the word");
    io::stdin()
        .read_line(&mut word_in)
        .expect("Failed to read submission");

    let entry = word_in;

    let output = find_words(entry);

    println!("{:?}", output);
}

fn find_words(words: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for word in words.iter() {
        let wordset: HashSet<char> = word.to_lowercase().chars().collect();

        let qrow: HashSet<char> = String::from("qwertyuiop").chars().collect();
        let arow: HashSet<char> = String::from("asdfghjkl").chars().collect();
        let zrow: HashSet<char> = String::from("zxcvbnm").chars().collect();

        if (wordset.is_subset(&qrow) || wordset.is_subset(&arow) || wordset.is_subset(&zrow)) {
            results.push(word.to_string());
        }

    }
    return results;
}