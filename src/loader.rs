use crate::connectors;
use crate::store;
use mongodb::{Client, ThreadedClient};
use std::thread;
use std::time::Duration;

const DEFAULT_INTERVAL_IN_SECS: u64 = 30;

pub fn bike_loader() -> () {
    println!("Loader starting...");
    let client = Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");
    let coll = client.db("city-cycling");
    println!("Loader started");
    loop {
        println!("Loading bike points...");
        let bike_points = connectors::london::get_santander_bike_point();
        store::store_bike_points(&coll, &bike_points);
        println!("{} bike points stored", bike_points.len());
        thread::sleep(Duration::from_secs(DEFAULT_INTERVAL_IN_SECS));
    }
}
