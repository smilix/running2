use sqlx::arguments::Arguments;
use sqlx::sqlite::{SqliteArguments, SqliteQueryAs};
use sqlx::SqlitePool;

pub async fn get_last_inserted_id(pool: &SqlitePool) -> Result<i32, sqlx::Error> {
    let rec: (i32, ) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(pool)
        .await?;

    Ok(rec.0)
}


pub struct DynUpdate {
    sql: String,
    pub bind_values: SqliteArguments,
}

impl DynUpdate {
    pub fn new() -> Self {
        return DynUpdate {
            sql: String::new(),
            bind_values: SqliteArguments::default(),
        };
    }

    pub fn add<T>(&mut self, field_name: &str, value: T)
        where
            T: sqlx::encode::Encode<sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite>,
    {
        self.sql.push_str(format!(" {} = ?,", field_name).as_str());
        self.bind_values.add(value);
    }

    pub fn add_option<T>(&mut self, field_name: &str, value: Option<T>)
        where
            T: sqlx::encode::Encode<sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite>,
    {
        if value.is_some() {
            self.add(field_name, value.unwrap());
        }
    }

    pub fn has_updates(&self) -> bool {
        return self.sql.len() > 0;
    }

    pub fn to_query_str(&self, prefix: &str, suffix: &str) -> String {
        let mut sql = String::from(prefix);
        sql.push_str(self.sql.as_str());
        if self.has_updates() {
            // remove the last comma
            sql.remove(sql.len() - 1);
        }
        sql.push_str(suffix);

        return sql;
    }
}