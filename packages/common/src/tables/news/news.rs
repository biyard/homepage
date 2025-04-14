use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/news", table = news)]
pub struct News {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub category: String,
    #[api_model(summary)]
    pub title: String,
    #[api_model(summary)]
    pub image: String,
    #[api_model(summary)]
    #[validate(length(max = 350))]
    pub contents: String,
    #[api_model(summary)]
    pub main: bool,
}
