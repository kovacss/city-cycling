
use crate::store;
use crate::connectors;
use std::thread;
use std::time::Duration;

pub fn bike_loader() -> () {
    let db_pool = store::pool();
    let st = store::Store { pool: db_pool };
    println!("Loader loading...");
    loop {
        println!("Loading bike...");
        let bike_points = connectors::get_london_bike_points();
        st.store_bike_points(bike_points);
        thread::sleep(Duration::from_millis(5000));
    }
}
