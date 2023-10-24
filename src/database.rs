use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;

use self::models::File;

pub mod models;
pub mod schema;

type DBPool = Pool<ConnectionManager<SqliteConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        let _ = dotenv();

        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(db_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Database connection error : Failed to create Pool");

        Database { pool }
    }

    pub fn get_file(&mut self, id: String) -> Option<File> {
        use self::schema::files;

        let mut conn = self.pool.get().ok()?;
        if let Ok(record) = files::table.find(id).get_result::<File>(&mut conn) {
            Some(record)
        } else {
            None
        }
    }
}
