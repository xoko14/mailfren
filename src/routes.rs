use std::{net::SocketAddr, str::from_utf8, collections::HashMap};

use axum::{extract::{State, Host, Path, Request, ConnectInfo}, Json, response::{IntoResponse, Response}, http::{HeaderMap, header}, body::Body};
use sqlx::SqlitePool;

use crate::{schemas::{FrenRequest, Fren, Greeting}, errors::{MailfrenResult, MailfrenError}, database};

static PIXEL: &[u8; 563] = include_bytes!("../assets/pixel.png");

pub async fn create_fren(State(db): State<SqlitePool>, Host(host): Host, Json(fren_request): Json<FrenRequest>) -> MailfrenResult<Json<Fren>>{
    let fren_id = database::create_fren(&db, fren_request).await?;
    let fren = get_fren_with_url(&db, fren_id, host).await?;
    Ok(Json(fren.ok_or(MailfrenError::InternalServerError)?))
}

pub async fn get_fren(State(db): State<SqlitePool>, Path(id): Path<i64>, Host(host): Host) -> MailfrenResult<Json<Fren>>{
    let fren = get_fren_with_url(&db, id, host).await?;
    Ok(Json(fren.ok_or(MailfrenError::EntityNotFound("Fren".to_owned(), id))?))
}

pub async fn greet_fren(ConnectInfo(addr): ConnectInfo<SocketAddr>, State(db): State<SqlitePool>, Path(id): Path<i64>, request: Request) -> Response{
    let fren = database::get_fren(&db, id).await.unwrap_or(None);

    if fren.is_some(){
        let ip = addr.to_string();
        let headers = request.headers()
            .iter()
            .map(|(k, v)| (k.as_str().to_owned(), from_utf8(v.as_bytes()).unwrap_or(&String::new()).to_owned()))
            .collect::<HashMap<String, String>>();
        let headers =  serde_json::to_string(&headers).unwrap_or(String::new());

        _ = database::create_greeting(&db, id, ip, headers).await
    }

    ([
        (header::CONTENT_TYPE, "image/png")
    ], PIXEL).into_response()
}

pub async fn get_greetings(State(db): State<SqlitePool>, Path(id): Path<i64>) -> MailfrenResult<Json<Vec<Greeting>>>{
    let greetings = database::get_greetings(&db, id).await?;
    Ok(Json(greetings))
}

async fn get_fren_with_url(db: &SqlitePool, id: i64, host: String) -> MailfrenResult<Option<Fren>>{
    let fren = database::get_fren(db, id).await?;
    Ok(fren.map(|mut f|{
        f.url = format!("https://{}/v1/frens/{}/image.png", host, f.id);
        f
    }))
}

pub async fn fallback() -> impl IntoResponse{
    MailfrenError::NotFound
}