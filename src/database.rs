use std::fmt::format;

use sqlx::{Connection, SqlitePool, Pool, Sqlite, AnyPool};

use crate::{schemas::{FrenRequest, Fren, Greeting}, errors::MailfrenResult};

pub async fn get_pool() -> Pool<Sqlite>{
    SqlitePool::connect("sqlite://database.db").await.unwrap()
}

pub async fn create_fren(db: &SqlitePool,fren_request: FrenRequest) -> MailfrenResult<i64>{
    let res = sqlx::query!(
        "insert into fren (code, description, custom_image)
        values (?, ?, ?)",
        fren_request.code,
        fren_request.description,
        fren_request.custom_image
    ).execute(db)
    .await?;
    Ok(res.last_insert_rowid())
}

pub async fn get_fren(db: &SqlitePool, id: i64) -> MailfrenResult<Option<Fren>>{
    let fren = sqlx::query!(
        "select * from fren where id = ?",
        id
    ).fetch_optional(db)
    .await?;

    Ok(fren.map(|r| Fren { id: r.id, code: r.code, description: r.description, url: String::new()}))
}

pub async fn create_greeting(db: &SqlitePool, fren_id: i64, ip: String, headers: String) -> MailfrenResult<()>{
    sqlx::query!(
        "insert into greeting (date, ip, headers, fren_id)
        values (DATETIME('now'), ?, ?, ?)",
        ip, headers, fren_id
    ).execute(db)
    .await?;
    Ok(())
}

pub async fn get_greetings(db: &SqlitePool, fren_id: i64) -> MailfrenResult<Vec<Greeting>>{
    let greetings= sqlx::query_as!(
        Greeting,
        "select id, date, ip, headers from greeting where fren_id = ?",
        fren_id
    ).fetch_all(db)
    .await?;

    Ok(greetings)
}