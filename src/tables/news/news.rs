use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/api/news", table = news)]
pub struct News {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    pub category: String,
    pub title: String,
    pub image: String,
    #[validate(length(max = 350))]
    pub contents: String,
    pub main: bool,
}
