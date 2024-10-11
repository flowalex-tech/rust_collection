use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let reader = BufReader::new(File::open("files/lines.txt").expect("Cannot open File: lines.txt"));

    let mut count = 1;

    for i in reader.lines() {
        if count == 10 {
            println!("{:?}", i);
            count += 1;
            //println!("{:?}", count);
        }
        else {
            count +=1;
            //println!("{:?}", count);
        }
    }
}