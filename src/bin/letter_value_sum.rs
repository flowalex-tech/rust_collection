use std::collections::HashMap;
use std::env;

fn letter_value_sum(word: &str, dict: &HashMap<char, usize>) -> usize {
    word.to_lowercase()
        .chars()
        .map(|ch| dict.get(&ch).unwrap_or(&0usize))
        .sum()
}


fn main() {
    let dict: HashMap<char, usize> = ('a'..='z')
        .enumerate()
        .map(|tuple| (tuple.1,tuple.0 +1 ))
        .collect();

    let args = env::args().skip(1);

    let len = args.len();

    match len {
        0usize => println!("No argument provided"),
        _ => {
            for (ind,arg) in args.enumerate() {
                println!("{}. {} {}", ind + 1, &arg, letter_value_sum(&arg, &dict));
            }
        }
    }
}