fn main() {
    let count: usize = (300..=400)
        .map(|n| n.to_string())
        .map(|s| s.chars().filter(|&c| c == '3').count())
        .sum();

    println!("The number 3 appears {} times between 300 and 400", count);
}