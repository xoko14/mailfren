use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FrenRequest {
    pub code: String,
    pub description: String,
    pub custom_image: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Fren {
    pub id: i64,
    pub code: String,
    pub description: String,
    pub url: String
}

#[derive(Serialize, Deserialize)]
pub struct Greeting {
    pub id: i64,
    pub date: NaiveDateTime,
    pub ip: String,
    pub headers: String,
}