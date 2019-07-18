#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::json::Json;
use std::thread;

mod bike_point;
mod loader;
mod redis_wrapper;
mod store;
mod connectors;

use bike_point::BikePoint;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/bikes", format = "json")]
fn get_bike_points(_connection: redis_wrapper::DbConn) -> Json<Vec<BikePoint>> {
    let bike_points = store::Store::get_bike_points(_connection);
    Json(bike_points)
}

fn main() {
    thread::spawn(|| {
        loader::bike_loader();
    });
    rocket::ignite()
        .manage(store::pool())
        .mount("/", routes![index])
        .mount("/api", routes![get_bike_points])
        .launch();
}
