use crate::bike_point::BikePoint;
use crate::bike_point::Position;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

type AdditionalProps = Vec<HashMap<String, Value>>;

#[derive(Serialize, Deserialize, Debug)]
struct SantanderBikePoint {
    id: String,
    #[serde(rename = "commonName")]
    common_name: String,
    #[serde(rename = "additionalProperties")]
    additional_properties: AdditionalProps,
    #[serde(rename = "lat")]
    latitude: f32,
    #[serde(rename = "lon")]
    longitude: f32,
}

pub fn get_santander_bike_point() -> Vec<BikePoint> {
    let file_content = fs::read_to_string("./bikepoint.json");
    let bike_points: Vec<SantanderBikePoint> =
        serde_json::from_str(file_content.unwrap().as_str()).unwrap();
    map_to_standard_bike_point(bike_points)
}

fn map_to_standard_bike_point(bike_points: Vec<SantanderBikePoint>) -> Vec<BikePoint> {
    return bike_points
        .into_iter()
        .map(|point: SantanderBikePoint| {
            let props = point.additional_properties;
            let nb_docks: i16 = get_additional_prop_as_int(&props, "NbDocks");
            let nb_empty_docks: i16 = get_additional_prop_as_int(&props, "NbEmptyDocks");
            BikePoint {
                id: point.id,
                common_name: point.common_name,
                temporary: false,
                nb_docks,
                nb_empty_docks,
                position: Position {
                    longitude: point.longitude,
                    latitude: point.latitude,
                },
            }
        })
        .collect();
}

fn get_additional_prop<'a>(props: &'a AdditionalProps, key: &'static str) -> &'a Value {
    let object: Vec<&HashMap<String, Value>> = props
        .iter()
        .filter(|&prop| prop.get("key").unwrap() == key)
        .collect();
    object[0].get("value").unwrap()
}

fn get_additional_prop_as_int<'a>(props: &'a AdditionalProps, key: &'static str) -> i16 {
    get_additional_prop(props, key)
        .as_str()
        .unwrap_or_default()
        .parse()
        .unwrap_or_default()
}
