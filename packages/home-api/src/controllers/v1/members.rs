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
pub struct MemberPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct MemberController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl MemberController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        _param: MemberQuery,
    ) -> Result<QueryResponse<MemberSummary>> {
        let mut total_count = 0;
        let items: Vec<MemberSummary> = MemberSummary::query_builder()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}

impl MemberController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_member))
            .with_state(self.clone()))
    }

    pub async fn get_member(
        State(ctrl): State<MemberController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<MemberParam>,
    ) -> Result<Json<MemberGetResponse>> {
        tracing::debug!("list_member {:?}", q);

        match q {
            MemberParam::Query(param) => Ok(Json(MemberGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
            // MemberParam::Read(param)
            //     if param.action == Some(MemberReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(MemberGetResponse::Read(res)))
            // }
        }
    }
}
