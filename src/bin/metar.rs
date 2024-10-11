use metar::Metar;
use chrono::{NaiveDateTime, TimeZone, Utc};

fn main() {
    let metar_report = "KMSP 241053Z 13006KT 10SM BKN080 OVC100 13/09 A2982 RMK AO2 SLP098 T01280089";

    let metar: Metar = Metar::parse(metar_report).unwrap();

    let timestamp = &metar_report[5..12];
    let naive_dt = NaiveDateTime::parse_from_str(
        timestamp,
        "%d%H%M%z"
    );

    let dt = Utc::from_local_datetime(&naive_dt).unwrap();

    println!("Airport: {}", metar.station);
    println!("Date: {}", dt.format("%Y-%m-%d"));
    println!("Time: {}", dt.format("%H:%M%:SZ"));
    println!("Wind: {:?} kt from {:?}", metar.wind.speed, metar.wind.dir);
    println!("Visibility: {:?}", metar.visibility);
    println!("Clouds: {:?}", metar.clouds);
    println!("Temperature: {:?}", metar.temperature);
    println!("Dewpoint: {:?}", metar.dewpoint);
    println!("Altimeter: {:?} in HG", metar.pressure);
    println!("Remarks: {:?}", metar.remarks);

}

// This will just print the metar report completly
// fn main() {
//
//     let r = Metar::parse(metar).unwrap();
//     println!("{:#?}", r);
// }
