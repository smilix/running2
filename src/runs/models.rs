use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use sqlx::arguments::Arguments;
use sqlx::sqlite::{SqliteQueryAs};

use crate::db_utils::{get_last_inserted_id, DynUpdate};
use crate::errors::ApiError;
use crate::common::time_now;

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Run {
    #[sqlx(rename = "Id")]
    #[serde(default)] // = optional
    pub id: i32,
    // in meter
    #[sqlx(rename = "Length")]
    pub length: i32,
    // unix timestamp
    #[sqlx(rename = "Date")]
    pub date: i64,
    // in seconds
    #[sqlx(rename = "TimeUsed")]
    pub time_used: i64,
    #[sqlx(rename = "Comment")]
    pub comment: String,

    #[sqlx(rename = "Created")]
    #[serde(default)] // = optional
    pub tscreated: i64,

    #[sqlx(rename = "ShoeId")]
    pub shoe_id: i32,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct UpdateRun {
    pub length: Option<i32>,
    pub date: Option<i64>,
    pub time_used: Option<i64>,
    pub comment: Option<String>,
    pub shoe_id: Option<i32>,
}

impl Run {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Run>, sqlx::Error> {
        let runs = sqlx::query_as::<_, Run>(
            "SELECT * FROM Runs ORDER BY Date DESC",
        )
            .fetch_all(pool)
            .await?;

        Ok(runs)
    }

    pub async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<Option<Run>, sqlx::Error> {
        let run = sqlx::query_as::<_, Run>(
            "SELECT * FROM Runs WHERE id = ?"
        )
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(run)
    }

    pub async fn create(run: Run, pool: &SqlitePool) -> Result<i32, sqlx::Error> {
        let mut tx = pool.begin().await?;
        sqlx::query("INSERT INTO Runs (Length, Date, TimeUsed, Comment, Created, ShoeId) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(run.length)
            .bind(run.date)
            .bind(run.time_used)
            .bind(run.comment)
            .bind(time_now())
            .bind(run.shoe_id)
            .execute(&mut tx)
            .await?;
        tx.commit().await?;

        return get_last_inserted_id(pool).await;
    }

    pub async fn update(id: i32, update: UpdateRun, pool: &SqlitePool) -> Result<Run, ApiError> {
        let mut dyn_update = DynUpdate::new();

        dyn_update.add_option("Length", update.length);
        dyn_update.add_option("Date", update.date);
        dyn_update.add_option("TimeUsed", update.time_used);
        dyn_update.add_option("Comment", update.comment);
        dyn_update.add_option("ShoeId", update.shoe_id);

        let sql = dyn_update.to_query_str("update Runs set  ", " where id = ?");
        dyn_update.bind_values.add(id);

        let mut tx = pool.begin().await?;
        let q = sqlx::query(sql.as_str())
            .bind_all(dyn_update.bind_values)
            .execute(&mut tx)
            .await?;

        if q != 1 {
            tx.rollback().await?;
            return Err(ApiError::NotFound("No run found for id.".to_string()));
        }

        tx.commit().await?;

        // get a fresh run from the db
        return Ok(Run::find_by_id(id, pool)
            .await?
            .unwrap());
    }

    pub async fn delete(id: i32, pool: &SqlitePool) -> Result<(), ApiError> {
        let mut tx = pool.begin().await?;
        let q = sqlx::query("DELETE FROM Runs where Id = ?")
            .bind(id)
            .execute(&mut tx)
            .await?;

        if q != 1 {
            tx.rollback().await?;
            return Err(ApiError::NotFound("No run found for id.".to_string()));
        }

        tx.commit().await?;

        return Ok(());
    }
}
