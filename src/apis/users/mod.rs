#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct KeepUpdatesRequest {
    pub email: String,
}

#[server(endpoint = "/users/keep-updates", input = PostUrl, output = Json)]
pub async fn keep_updates(req: KeepUpdatesRequest) -> Result<(), ServerFnError> {
    tracing::debug!("/users/keep-updates: {:?}", req);
    use crate::models::user::User;

    let user = User {
        id: req.email.to_string(),
        email: req.email.to_string(),
        role: crate::models::user::Role::Subscriber,
    };

    let cli = easy_dynamodb::Client::new(
        slog::Logger::root(
            slog::Discard,
            slog::o!("service" => "homepage", "api"=>"/users/keep-updates"),
        ),
        option_env!("AWS_ACCESS_KEY_ID")
            .expect("AWS_ACCESS_KEY_ID is required")
            .to_string(),
        option_env!("AWS_SECRET_ACCESS_KEY")
            .expect("AWS_SECRET_ACCESS_KEY is required")
            .to_string(),
        option_env!("AWS_REGION")
            .unwrap_or("ap-northeast-2")
            .to_string(),
        option_env!("TABLE_NAME")
            .expect("TABLE_NAME is required")
            .to_string(),
        "id".to_string(),
        None,
        None,
    );

    tracing::debug!(
        "creating user to {}",
        option_env!("TABLE_NAME")
            .expect("TABLE_NAME is required")
            .to_string(),
    );
    match cli.create(user.clone()).await {
        Ok(_) => {
            tracing::debug!("user created: {:?}", user);
        }
        Err(e) => {
            tracing::error!("failed to create user: {:?}", e);
            return Err(ServerFnError::ServerError(e.to_string()));
        }
    };

    Ok(())
}
