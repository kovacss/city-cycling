use bson;

use crate::bike_point::BikePoint;
use mongodb::db::ThreadedDatabase;

const LONDON_COLLECTION: &str = "london";

pub fn store_bike_points(db: &mongodb::db::Database, new_bike_points: &Vec<BikePoint>) {
    let value = bson::to_bson(&new_bike_points)
        .expect("Failed to serialize bike_points to bson.")
        .as_array()
        .expect("Failed to get document.")
        .clone();
    let vec = value
        .iter()
        .map(|d| d.as_document().unwrap().clone())
        .collect();
    db.collection(LONDON_COLLECTION).insert_many(vec, None);
    println!("BikePoints store ...");
}

pub fn get_bike_points(db: &mongodb::db::Database) -> Vec<BikePoint> {
    println!("Fetching From BikePoints from store ...");

    let mut bike_points: Vec<BikePoint> = vec![];
    let cursor = db.collection(LONDON_COLLECTION).find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            let t = bson::from_bson(bson::Bson::Document(item)).unwrap();
            bike_points.push(t);
        } else {
            println!("item not found");
        }
    }
    bike_points
}
