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
            .order_by_role_asc()
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
        }
    }
}

#[cfg(test)]
mod member_tests {
    use crate::tests::{TestContext, setup};

    use super::*;

    #[tokio::test]
    async fn test_list_members() {
        let TestContext { pool, endpoint, .. } =
            setup().await.expect("Failed to setup test context");

        let repo = Member::get_repository(pool.clone());
        for Member { name, image, role, email, web, linkedin, github, description, .. } in vec![
            Member {
                id: 1,
                created_at: 0,
                updated_at: 0,
                name: "Summer".to_string(),
                role: MemberRole::Founder,
                image: "https://dev.biyard.co/members/summer.png".to_string(),
                description: "Our founder and CEO, Summer is an educator in the Korean Web3 industry, specializing in industry convergence and DAO technology and applications in the context of blockchain and Web3.0. In the UK, she is also a Venture Partner at Simsan Ventures. Simsan Ventures is a London-based venture capital firm focused on early stage and deep tech companies. In addition, she is the CIO of the Korea office of Simsan Ventures.".to_string(),
                email: "summer@biyard.co".to_string(),
                github: None,
                linkedin: Some("https://www.linkedin.com/in/summerhyejinpark/".to_string()),
                web: None,
            },
            Member {
                id: 1,
                created_at: 0,
                updated_at: 0,
                name: "Miner".to_string(),
                role: MemberRole::CeoAndCto,
                image: "https://dev.biyard.co/members/miner.png".to_string(),
                description: "Miner obtained his PhD in cryptography and computer security. He has published several SCI(E) papers in the field of security. He led the development of the messenger part of a game platform with 75 million concurrent users, and served as the development leader and a product owner of an EVM-based blockchain cloud platform. In addition, he has contributed various open source activities both private and public blockchains.".to_string(),
                email: "miner@biyard.co".to_string(),
                github: Some("https://github.com/hackartists".to_string()),
                linkedin: Some("https://www.linkedin.com/in/hackartist/".to_string()),
                web: Some("https://www.hackartist.io".to_string()),
            },
        ] {
            repo.insert(name, image, role, email, web, linkedin, github, description)
            .await
            .expect("Failed to insert member");
        }

        let members = Member::get_client(&endpoint)
            .query(MemberQuery::new(10))
            .await
            .expect("Failed to query members");

        assert_eq!(members.total_count, 2);
        assert_eq!(members.items.len(), 2);
        assert_eq!(members.items[0].name, "Summer", "{:?}", members);
        assert_eq!(members.items[1].name, "Miner", "{:?}", members);
        assert_eq!(members.items[0].role, MemberRole::Founder, "{:?}", members);
        assert_eq!(
            members.items[1].role,
            MemberRole::CeoAndCto,
            "{:?}",
            members
        );
    }
}
