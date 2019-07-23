#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::json::Json;
use std::thread;

mod bike_point;
mod connectors;
mod loader;
mod store;

use bike_point::BikePoint;

#[macro_use(bson, doc)]
extern crate bson;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/bikes", format = "json")]
fn get_bike_points(_connection: MyDatabase) -> Json<Vec<BikePoint>> {
    let bike_points = store::get_bike_points(&*_connection);
    Json(bike_points)
}

#[rocket_contrib::database("city-cycling")]
struct MyDatabase(mongodb::db::Database);

fn main() {
    thread::spawn(|| {
        loader::bike_loader();
    });
    rocket::ignite()
        .attach(MyDatabase::fairing())
        .mount("/", routes![index])
        .mount("/api", routes![get_bike_points])
        .launch();
}
