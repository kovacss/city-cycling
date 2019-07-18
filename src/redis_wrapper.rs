use core::ops::Deref;
use rocket::request::{self, FromRequest};
use rocket::http::Status;
use rocket::{Request, State, Outcome};
use r2d2_redis::RedisConnectionManager;
use r2d2::{Pool, PooledConnection};

pub type RedisPool = Pool<RedisConnectionManager>;

pub type RedisConnection = PooledConnection<RedisConnectionManager>;

pub struct DbConn(pub RedisConnection);

// Our impl of FromRequest for our DbConn tuple-struct.
// This is what actually enables our connection pool to become
// a request guard.
// Docs: https://api.rocket.rs/rocket/request/trait.FromRequest.html
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = (); // Associated type, we must have an error we can `Debug`

    // This is our required method that does all the dirty work.
    // Because FromRequest is a "validation", we can put whatever logic we want in here
    // as long as it conforms to the function signature.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {

        // This next part is a little dense, but what it's doing is grabbing the 
        // `guard` property off of the `request` object. From there we have to give
        // it a type, which you'll see in this massive turbofish ::<<<<>>>.
        // ...Might be a world record :P

        // The outside most type is State, which is the managed state we will be registering
        // with our rocket app when we initialize it. Don't worry, we'll get to that soon enough,
        // but you'll have to trust me here.
         let pool = request.guard::<State<RedisPool>>()?;

        // Here were are using the `get()` method from the connection pool to grab 
        // the connection. If it's Ok, return the DbConn tuple-struct we made
        // wrapped in an `Outcome` to conform to the function signature.
        // If the `get()` returns an Error, we're returning a tuple with the
        // signature (SomeFailureType, ())
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}


// This is not immediately apparent, but because our connection is wrapped up
// in a PooledConnection, we have a few layers of indirection between
// the DbConn tuple-struct and the actual PgConnection.
//
// PooledConnection is a smart pointer referencing a connection.
// Doc: https://docs.rs/r2d2/0.7.4/r2d2/struct.PooledConnection.html
//
// We implement Deref because we want that PgConnection directly and it's behind a smart pointer.
// PooledConnection already implements Deref to do this, but our Managed State has a hold of
// DbConn, the wrapper!
//
// Implementing Deref for DbConn enables us to write `&*connection_variable` when we want
// to get at the actual connection.
// Deref Rust Docs: https://doc.rust-lang.org/std/ops/trait.Deref.html
impl Deref for DbConn {
    type Target = r2d2::PooledConnection<RedisConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}