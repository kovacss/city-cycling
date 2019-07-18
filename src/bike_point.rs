use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BikePoint {
    pub id: String,
    pub common_name: String,
    pub temporary: bool,
    pub nb_docks: i16,
    pub nb_empty_docks: i16,
    pub position: Position
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub latitude: f32,
    pub longitude: f32,
}
