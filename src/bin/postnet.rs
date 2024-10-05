extern crate csv;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

//TO DO: Added Check Digit
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::create("files/barcodes.txt");
    let mut reader = csv::Reader::from_path("files/address.csv").expect("Cannot open Addresses.csv");

    for line in reader.records() {
        let record = line?;
        let name = &record[0];
        let street = &record[1];
        let city = &record[2];
        let state = &record[3];
        let zip = &record[4];

        // Print to terminal
        println!("{}", name);
        println!("{}", street);
        println!("{}, {} {}", city, state, zip);
        print!("|");
        for i in zip.chars(){
            match i {
                '0' => print!("||:::"),
                '1' => print!(":::||"),
                '2' => print!("::|:|"),
                '3' => print!("::||:"),
                '4' => print!(":|::|"),
                '5' => print!(":|:|:"),
                '6' => print!(":||::"),
                '7' => print!("|:::|"),
                '8' => print!("|::|:"),
                '9' => print!("|:|::"),
                _ => (),
            };
        }
        print!("|");
        println!();

        // Write to File
        let mut output = OpenOptions::new()
            .append(true)
            .open("files/barcodes.txt")
            .unwrap();

        writeln!(output, "{}", name);
        writeln!(output, "{}", street);
        writeln!(output, "{}, {} {}", city, state, zip);
        write!(output, "|");
        for i in zip.chars(){
            match i {
                '0' => write!(output, "||:::"),
                '1' => write!(output, ":::||"),
                '2' => write!(output, "::|:|"),
                '3' => write!(output, "::||:"),
                '4' => write!(output, ":|::|"),
                '5' => write!(output, ":|:|:"),
                '6' => write!(output, ":||::"),
                '7' => write!(output, "|:::|"),
                '8' => write!(output, "|::|:"),
                '9' => write!(output, "|:|::"),
                _ => Ok(()),
            };
        }
        write!(output, "|");
        writeln!(output);

    }
    Ok(())
}