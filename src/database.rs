use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

use self::models::{File, Part};

pub mod models;
pub mod schema;

type DBPool = Pool<ConnectionManager<SqliteConnection>>;

/// Represent the type of file matching given ID
pub enum FileType {
    Split(usize),
    Monolithic(File),
    Inexistent,
}

pub struct Database {
    pool: DBPool,
}

impl Database {
    fn get_conn(&mut self) -> Option<PooledConnection<ConnectionManager<SqliteConnection>>> {
        Some(self.pool.get().ok()?)
    }

    pub fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(db_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Database connection error : Failed to create Pool");

        Database { pool }
    }

    pub fn get_part(&mut self, id: String) -> Option<Part> {
        use self::schema::parts;

        let mut conn = self.pool.get().ok()?;
        if let Ok(record) = parts::table.find(id.clone()).get_result(&mut conn) {
            Some(record)
        } else {
            eprintln!("No matching part with ID : {id}");
            None
        }
    }

    pub fn get_parts(&mut self, id: String) -> Vec<Part> {
        use self::schema::parts;

        let records = if let Ok(mut conn) = self.pool.get() {
            parts::table
                .filter(parts::file_id.eq(id))
                .get_results(&mut conn)
                .unwrap_or(vec![])
        } else {
            vec![]
        };

        records
    }

    pub fn delete_part(&mut self, id: String) -> Option<Part> {
        use self::schema::parts;

        let mut conn = self.get_conn()?;
        match diesel::delete(parts::table.filter(parts::id.eq(id.clone()))).get_result(&mut conn) {
            Ok(record) => Some(record),
            Err(err) => {
                eprintln!("{err}");
                None
            }
        }
    }

    /// Register file into database
    pub fn register_file(&mut self, file: File) -> Result<(), String> {
        use self::schema::files::dsl::*;

        if let Some(mut conn) = self.get_conn() {
            diesel::insert_into(files)
                .values(file)
                .execute(&mut conn)
                .map_err(|err| err.to_string())?;
            Ok(())
        } else {
            Err("Failed to connect to database".to_owned())
        }
    }

    /// Delete file and all of its parts.
    pub fn delete_file(&mut self, id: String) -> u32 {
        use self::schema::{files, parts};

        if let Some(mut conn) = self.get_conn() {
            let count = if let Ok(count) =
                diesel::delete(parts::table.filter(parts::file_id.eq(id.clone())))
                    .execute(&mut conn)
            {
                let _ = diesel::delete(files::table.filter(files::id.eq(id))).execute(&mut conn);
                count
            } else {
                0
            };
            return count.try_into().unwrap();
        }

        0
    }

    pub fn get_file(&mut self, id: String) -> Option<File> {
        use self::schema::files;

        let mut conn = self.get_conn()?;
        files::table.find(id).get_result(&mut conn).ok()
    }

    pub fn get_file_type(&mut self, id: String) -> FileType {
        use self::schema::{files, parts};

        if let Some(mut conn) = self.get_conn() {
            let parts_count: usize = parts::table
                .filter(parts::file_id.eq(id.clone()))
                .execute(&mut conn)
                .unwrap_or(0);

            if parts_count == 0 {
                if let Ok(record) = files::table.find(id).get_result::<File>(&mut conn) {
                    FileType::Monolithic(record)
                } else {
                    FileType::Inexistent
                }
            } else {
                FileType::Split(parts_count)
            }
        } else {
            FileType::Inexistent
        }
    }
}