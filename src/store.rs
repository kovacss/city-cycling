extern crate redis;

extern crate r2d2_redis;

use crate::redis_wrapper;
use r2d2::PooledConnection;
use r2d2_redis::redis::Commands;
use r2d2_redis::{r2d2, RedisConnectionManager};

use crate::bike_point::BikePoint;

const REDIS_KEY: &'static str = "UK_LONDON";

const REDIS_ADDRESS: &'static str = "redis://127.0.0.1/";

// Pool initiation.
// Call it starting an app and store a pul as a rocket managed state.
pub fn pool() -> r2d2::Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new(REDIS_ADDRESS).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    pool
}

// Where T is RedisConnectionManager
pub struct Store {
    pub pool: r2d2::Pool<RedisConnectionManager>,
}

impl Store {
    fn get_connection(&self) -> PooledConnection<RedisConnectionManager> {
        self.pool.get().unwrap()
    }

    pub fn store_bike_points(&self, new_bike_points: Vec<BikePoint>) {
        let conn = self.get_connection();
        let value = serde_json::to_string(&new_bike_points).unwrap();
        let _: () = conn.set(REDIS_KEY, value).unwrap();
        println!("BikePoints store ...");
    }

    pub fn get_bike_points(_connection: redis_wrapper::DbConn) -> Vec<BikePoint> {
        println!("Fetching From BikePoints from store ...");
        let t: String = _connection.get(REDIS_KEY).unwrap();
        serde_json::from_str(&t).unwrap()
    }
}
