use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
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
    fn get_conn(&mut self) -> Option<PooledConnection<ConnectionManager<SqliteConnection>>> {
        Some(self.pool.get().ok()?)
    }

    pub fn new() -> Self {
        let _ = dotenv();

        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(db_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Database connection error : Failed to create Pool");

        Database { pool }
    }

    pub fn get_part(&mut self, part_id: String) -> Option<File> {
        use self::schema::files;

        let mut conn = self.pool.get().ok()?;
        if let Ok(record) = files::table.find(part_id.clone()).get_result(&mut conn) {
            Some(record)
        } else {
            eprintln!("No matching part with ID : {part_id}");
            None
        }
    }

    pub fn get_parts(&mut self, file_id: String) -> Vec<File> {
        use self::schema::files;

        let records = if let Ok(mut conn) = self.pool.get() {
            files::table
                .filter(files::file_id.eq(file_id))
                .get_results(&mut conn)
                .unwrap_or(vec![])
        } else {
            vec![]
        };

        records
    }

    pub fn delete(&mut self, part_id: String) -> Option<File> {
        use self::schema::files::dsl::*;

        let mut conn = self.get_conn()?;
        match diesel::delete(files.filter(id.eq(part_id.clone()))).get_result(&mut conn) {
            Ok(record) => Some(record),
            Err(err) => {
                eprintln!("{err}");
                None
            }
        }
    }

    /// Delete all parts associated with the given file ID
    pub fn delete_file(&mut self, id: String) -> u32 {
        use self::schema::files;

        if let Some(mut conn) = self.get_conn() {
            let count = if let Ok(count) =
                diesel::delete(files::table.filter(files::file_id.eq(id))).execute(&mut conn)
            {
                count
            } else {
                0
            };
            return count.try_into().unwrap();
        }

        0
    }
}
