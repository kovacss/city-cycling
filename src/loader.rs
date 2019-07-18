use crate::connectors;
use crate::store;
use std::thread;
use std::time::Duration;

const DEFAULT_INTERVAL_IN_SECS: u64 = 30;

pub fn bike_loader() -> () {
    let db_pool = store::pool();
    let st = store::Store { pool: db_pool };
    println!("Loader started");
    loop {
        println!("Loading bike points...");
        let bike_points = connectors::get_london_bike_points();
        st.store_bike_points(&bike_points);
        println!("{} bike points stored", bike_points.len());
        thread::sleep(Duration::from_secs(DEFAULT_INTERVAL_IN_SECS));
    }
}
