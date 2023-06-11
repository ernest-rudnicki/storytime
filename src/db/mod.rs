use std::env;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> PostgresPool{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let migr = ConnectionManager::<PgConnection>::new(database_url);
    
    Pool::builder()
    .build(migr)
    .expect("Failed to create pool.")
}