pub mod user;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn connect_to_database() -> DBPool {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
