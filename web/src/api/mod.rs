use actix_web::web;
use serde::Serialize;
use serde_derive::{Deserialize, Serialize};

pub mod battle;
pub mod mon;
pub mod series;
pub mod trainer;

pub const TOURNAMENT_ID: i32 = 1;

#[derive(Serialize)]
pub struct PagedResponse<T: Serialize> {
    pub data: T,
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

impl<T: Serialize> PagedResponse<T> {
    pub fn new(data: T, limit: i64, offset: i64, total: i64) -> Self {
        PagedResponse {
            data,
            limit,
            offset,
            total,
        }
    }
}

fn default_limit() -> i64 {
    10
}

#[derive(Deserialize)]
pub struct PagedQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

pub(crate) fn scope() -> actix_web::Scope {
    web::scope("/api/v1")
        .service(mon::scope())
        .service(series::scope())
        .service(trainer::scope())
        .service(battle::scope())
}
