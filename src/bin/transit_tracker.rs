use comfy_table::Table;
use reqwest::Client;
use serde::Deserialize;
use serde_json::from_str;


#[derive(Deserialize)]
struct Stop {
    stop_id: i32,
    latitude: f64,
    longitude: f64,
    description: String,
}

#[derive(Deserialize)]
struct Departure {
    actual: bool,
    trip_id: String,
    stop_id: i32,
    departure_text: String,
    departure_time: i64,
    description: String,
    route_id: String,
    route_short_name: String,
    direction_id: i32,
    direction_text: String,
    schedule_relationship: String,
}

#[derive(Deserialize)]
struct Response {
    stops: Vec<Stop>,
    alerts: Vec<String>,
    departures: Vec<Departure>,
}

#[tokio::main]
async fn main() {
    let sb_client = Client::new();
    let sb_response = sb_client.get("https://svc.metrotransit.org/nextrip/51409?format=json").send().await.unwrap();
    let sb_json = sb_response.text().await.unwrap();
    let sb_data: Response = from_str(&sb_json).unwrap();

    let mut sb_table = Table::new();

    sb_table.set_header(vec!["Route", "Due In", "Destination"]);

    // Do something with the parsed data
    for departure in sb_data.departures {
        //println!("{:?} {:?} {:?}", departure.route_short_name, departure.departure_text, departure.description);
        sb_table.add_row(vec![departure.route_short_name, departure.departure_text, departure.description]);
    }

    for stop in sb_data.stops{
        let station = stop.description;
        println!("{:} Right Platform", station);
    }

    println!("{sb_table}");

    let nb_client = Client::new();
    let nb_response = nb_client.get("https://svc.metrotransit.org/nextrip/51424?format=json").send().await.unwrap();
    let nb_json = nb_response.text().await.unwrap();
    let nb_data: Response = from_str(&nb_json).unwrap();

    let mut nb_table = Table::new();

    nb_table.set_header(vec!["Route", "Due In", "Destination"]);

    // Do something with the parsed data
    for departure in nb_data.departures {
        //println!("{:?} {:?} {:?}", departure.route_short_name, departure.departure_text, departure.description);
        nb_table.add_row(vec![departure.route_short_name, departure.departure_text, departure.description]);
    }

    for stop in nb_data.stops{
        let station = stop.description;
        println!("{:} Left Platform", station);
    }


    println!("{nb_table}");
}


//reqwest = { version = "0.12.5", features = ["blocking", "json"] }
//serde = { version = "1", features = ["derive"] }
//comfy-table = "7.1.1"
//json = "0.12.4"
//serde_json = "1.0.128"
//tokio = { version = "1.40.0", features = ["full"] }