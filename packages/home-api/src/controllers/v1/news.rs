use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::get,
    },
};
use by_types::QueryResponse;
use common::*;
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct NewsPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct NewsController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl NewsController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        _param: NewsQuery,
    ) -> Result<QueryResponse<NewsSummary>> {
        let mut total_count = 0;
        let mut items: Vec<NewsSummary> = NewsSummary::query_builder()
            .limit(1)
            .main_is_true()
            .query()
            .map(NewsSummary::from)
            .fetch_all(&self.pool)
            .await?;

        let ex: Vec<NewsSummary> = NewsSummary::query_builder()
            .limit(3)
            .main_is_false()
            .query()
            .map(NewsSummary::from)
            .fetch_all(&self.pool)
            .await?;

        sqlx::query("SELECT COUNT(*) as total_count FROM news")
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();
            })
            .fetch_one(&self.pool)
            .await?;

        items.extend(ex);

        Ok(QueryResponse { total_count, items })
    }
}

impl NewsController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_news))
            .with_state(self.clone()))
    }

    pub async fn get_news(
        State(ctrl): State<NewsController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<NewsParam>,
    ) -> Result<Json<NewsGetResponse>> {
        tracing::debug!("list_news {:?}", q);

        match q {
            NewsParam::Query(param) => {
                Ok(Json(NewsGetResponse::Query(ctrl.query(auth, param).await?)))
            } // NewsParam::Read(param)
              //     if param.action == Some(NewsReadActionType::ActionType) =>
              // {
              //     let res = ctrl.run_read_action(auth, param).await?;
              //     Ok(Json(NewsGetResponse::Read(res)))
              // }
        }
    }
}
