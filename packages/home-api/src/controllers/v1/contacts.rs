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
pub struct ContactPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct ContactController {
    repo: ContactRepository,
}

impl ContactController {
    async fn submit(
        &self,
        _auth: Option<Authorization>,
        ContactSubmitRequest {
            last_name,
            first_name,
            email,
            company_name,
            needs,
            help,
        }: ContactSubmitRequest,
    ) -> Result<Contact> {
        self.repo
            .insert(last_name, first_name, email, company_name, needs, help)
            .await
    }
}

impl ContactController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Contact::get_repository(pool);

        Self { repo }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_contact))
            .with_state(self.clone()))
    }

    pub async fn act_contact(
        State(ctrl): State<ContactController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<ContactAction>,
    ) -> Result<Json<Contact>> {
        tracing::debug!("act_contact {:?}", body);
        match body {
            ContactAction::Submit(param) => {
                let res = ctrl.submit(auth, param).await?;
                Ok(Json(res))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common::*;

    use crate::tests::{TestContext, setup};

    #[tokio::test]
    async fn test_submit_contact() {
        let TestContext {
            pool,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let last_name = "Doe".to_string();
        let first_name = "John".to_string();
        let email = format!("submit-contact-{}@example.com", now);
        let company_name = "Example Company".to_string();
        let needs = Need::GeneralInquiry;
        let help = "I need help with something.".to_string();

        Contact::get_client(&endpoint)
            .submit(
                last_name.clone(),
                first_name.clone(),
                email.clone(),
                company_name.clone(),
                needs,
                help.clone(),
            )
            .await
            .expect("Failed to submit contact");

        let doc = Contact::query_builder()
            .email_equals(email.clone())
            .query()
            .map(Contact::from)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch contact");

        assert_eq!(doc.last_name, last_name);
        assert_eq!(doc.first_name, first_name);
        assert_eq!(doc.email, email);
        assert_eq!(doc.company_name, company_name);
        assert_eq!(doc.needs, needs);
        assert_eq!(doc.help, help);
    }
}
