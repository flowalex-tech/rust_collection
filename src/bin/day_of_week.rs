use chrono::{TimeZone, Datelike, Date};
use std::io;

fn main() {
    let mut year_in = String::new();
    let mut month_in = String::new();
    let mut day_in = String::new();
    println!("Please enter the year");
    io::stdin()
        .read_line(&mut year_in)
        .expect("Failed to read line");
    println!("Please enter the month");
    io::stdin()
        .read_line(&mut month_in)
        .expect("Failed to read line");
    println!("Please enter the day");
    io::stdin()
        .read_line(&mut day_in)
        .expect("Failed to read line");
    let year = year_in.trim().parse().expect("Input not an integer");
    let month = month_in.trim().parse().expect("Input not an integer");
    let day = day_in.trim().parse().expect("Input not an integer");
    let dt = chrono::Local.ymd(year, month, day);
    print!("{:?}", dt.weekday());
}
    /*
    PROOF of working Challenge
    Please enter the year
    7032
    Please enter the month
    3
    Please enter the day
    26
    Mon%

    🕙[ 10:39:40 ] ❯ ./target/debug/dailyprogrammer
    Please enter the year
    2202
    Please enter the month
    12
    Please enter the day
    15
    Wed%

    🕙[ 10:39:50 ] ❯ ./target/debug/dailyprogrammer
    Please enter the year
    2017
    Please enter the month
    10
    Please enter the day
    30
    Mon%
    */