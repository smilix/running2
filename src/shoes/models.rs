use sqlx::{FromRow, SqlitePool};
use sqlx::sqlite::SqliteQueryAs;

use crate::db_utils::{get_last_inserted_id, DynUpdate};
use crate::errors::ApiError;
use crate::common::time_now;
use sqlx::arguments::Arguments;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Shoe {
    #[sqlx(rename = "Id")]
    #[serde(default)] // = optional
    pub id: i32,

    #[sqlx(rename = "Bought")]
    pub bought: i64,

    #[sqlx(rename = "Comment")]
    pub comment: String,

    #[sqlx(rename = "Created")]
    #[serde(default)] // = optional
    pub created: i64,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, FromRow)]
pub struct ShoeUsedView {
    #[sqlx(rename = "Id")]
    pub id: i32,

    #[sqlx(rename = "Bought")]
    pub bought: i64,

    #[sqlx(rename = "Comment")]
    pub comment: String,

    #[sqlx(rename = "Used")]
    pub used: i32,

    #[sqlx(rename = "TotalLength")]
    pub total_length: i32,
}

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct UpdateShoe {
    pub bought: Option<i64>,
    pub comment: Option<String>,
}

impl Shoe {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<ShoeUsedView>, sqlx::Error> {
        let sql = r"
select S.Id, S.Bought, S.Comment, count(R.Id) Used, ifnull(sum(R.Length), 0) TotalLength
from Shoes S left join Runs R on S.Id = R.ShoeId
group by S.Id, S.Bought, S.Comment
order by S.Bought desc
        ";
        let shoes = sqlx::query_as::<_, ShoeUsedView>(sql)
            .fetch_all(pool)
            .await?;

        Ok(shoes)
    }

    pub async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<Option<Shoe>, sqlx::Error> {
        let shoe = sqlx::query_as::<_, Shoe>(
            "SELECT * FROM Shoes WHERE id = ?"
        )
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(shoe)
    }

    pub async fn create(shoe: Shoe, pool: &SqlitePool) -> Result<i32, sqlx::Error> {
        let mut tx = pool.begin().await?;
        sqlx::query("INSERT INTO Shoes (Bought, Comment, Created) VALUES (?, ?, ?)")
            .bind(shoe.bought)
            .bind(shoe.comment)
            .bind(time_now())
            .execute(&mut tx)
            .await?;
        tx.commit().await?;

        return get_last_inserted_id(pool).await;
    }

    pub async fn update(id: i32, update: UpdateShoe, pool: &SqlitePool) -> Result<Shoe, ApiError> {
        let mut dyn_update = DynUpdate::new();

        dyn_update.add_option("Bought", update.bought);
        dyn_update.add_option("Comment", update.comment);

        let sql = dyn_update.to_query_str("UPDATE Shoes SET  ", " WHERE id = ?");
        dyn_update.bind_values.add(id);

        let mut tx = pool.begin().await?;
        let q = sqlx::query(sql.as_str())
            .bind_all(dyn_update.bind_values)
            .execute(&mut tx)
            .await?;

        if q != 1 {
            tx.rollback().await?;
            return Err(ApiError::NotFound("No shoe found for id.".to_string()));
        }

        tx.commit().await?;

        // get a fresh run from the db
        return Ok(Shoe::find_by_id(id, pool)
            .await?
            .unwrap());
    }

    pub async fn delete(id: i32, pool: &SqlitePool) -> Result<(), ApiError> {
        let mut tx = pool.begin().await?;
        let q = sqlx::query("DELETE FROM Shoes where Id = ?")
            .bind(id)
            .execute(&mut tx)
            .await?;

        if q != 1 {
            tx.rollback().await?;
            return Err(ApiError::NotFound("No shoe found for id.".to_string()));
        }

        tx.commit().await?;

        return Ok(());
    }
}
