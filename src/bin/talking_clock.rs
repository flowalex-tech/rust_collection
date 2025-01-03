fn number_to_words(n: u32) -> String {
    match n {
        0 => "twelve".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        21..=29 => format!("twenty {}", number_to_words(n - 20)),
        30 => "thirty".to_string(),
        31..=39 => format!("thirty {}", number_to_words(n - 30)),
        40 => "forty".to_string(),
        41..=49 => format!("forty {}", number_to_words(n - 40)),
        50 => "fifty".to_string(),
        51..=59 => format!("fifty {}", number_to_words(n - 50)),
        _ => unreachable!(),
    }
}

fn format_time(time: &str) -> String {
    let parts: Vec<&str> = time.split(':').collect();
    let hour: u32 = parts[0].parse().unwrap();
    let minute: u32 = parts[1].parse().unwrap();

    let (hour12, meridiem) = if hour == 0 {
        (12, "am")
    } else if hour < 12 {
        (hour, "am")
    } else if hour == 12 {
        (12, "pm")
    } else {
        (hour - 12, "pm")
    };

    let hour_word = number_to_words(hour12);

    let minute_word = if minute == 0 {
        String::new()
    } else if minute < 10 {
        format!(" oh {}", number_to_words(minute))
    } else {
        format!(" {}", number_to_words(minute))
    };

    format!("It's {}{} {}", hour_word, minute_word, meridiem)
}

fn main() {
    println!("{}", format_time("00:00"));
    println!("{}", format_time("01:30"));
    println!("{}", format_time("12:05"));
    println!("{}", format_time("14:01"));
    println!("{}", format_time("20:29"));
    println!("{}", format_time("21:00"));
}