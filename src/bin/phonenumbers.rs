extern crate phonelib;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub trait ToString {
    fn to_string(&self) -> String;
}

pub struct Country {
    pub name: &'static str,
    pub code: &'static str,
    pub phone_lengths: &'static [u8],
    pub prefix: u32,
}

fn main() {
    let reader = BufReader::new(File::open("files/phone_numbers.txt").expect("Cannot open File: Phonenumbers.txt"));

    for line in reader.lines() {

        let phone_number = line.unwrap();

        if  phonelib::is_valid_phone_number(phone_number.clone()) {
            println!("{} is a valid phone number.", phone_number);
        } else {
            println!("{} is not a valid phone number.", phone_number);
        }
    }
}

/*
Proof of Working Output
+1 987-123-4567 is a valid phone number.
+1 123 456 7890 is a valid phone number.
+1 (123) 456-7890 is a valid phone number.
+1 (651) 605-1240 is a valid phone number.
987-123-4567 is not a valid phone number.
123 456 7890 is not a valid phone number.
(123) 456-7890 is not a valid phone number.
*/