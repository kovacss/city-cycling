use crate::bike_point::BikePoint;

mod london;

pub fn get_london_bike_points() -> Vec<BikePoint> {
    london::get_santander_bike_point()
}