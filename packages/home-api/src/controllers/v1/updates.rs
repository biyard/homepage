use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{Extension, Json, extract::State, routing::post},
};
use common::*;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct UpdatePath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct UpdateController {
    repo: UpdateRepository,
}

impl UpdateController {
    async fn submit(
        &self,
        _auth: Option<Authorization>,
        UpdateSubmitRequest { email }: UpdateSubmitRequest,
    ) -> Result<Update> {
        self.repo.insert(email).await.map_err(|e| {
            tracing::error!("Update submit error: {:?}", e);
            Error::AlreadySubscribed
        })
    }
}

impl UpdateController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Update::get_repository(pool);

        Self { repo }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_update))
            .with_state(self.clone()))
    }

    pub async fn act_update(
        State(ctrl): State<UpdateController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<UpdateAction>,
    ) -> Result<Json<Update>> {
        tracing::debug!("act_update {:?}", body);
        match body {
            UpdateAction::Submit(param) => {
                let res = ctrl.submit(auth, param).await?;
                Ok(Json(res))
            }
        }
    }
}
