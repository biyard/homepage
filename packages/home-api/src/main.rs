use bdk::prelude::{by_axum::axum::Router, *};
use by_axum::{auth::authorization_middleware, axum::middleware};
use by_types::DatabaseConfig;
use common::*;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod config;
mod controllers {
    pub mod v1;
}

pub use controllers::*;

macro_rules! migrate {
    ($pool:ident, $($table:ident),* $(,)?) => {
        {
            $(
                let t = $table::get_repository($pool.clone());
                t.create_this_table().await?;
            )*
            $(
                let t = $table::get_repository($pool.clone());
                t.create_related_tables().await?;
            )*
        }
    };
}

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");

    migrate!(pool, Member, News, Contact);

    tracing::info!("Migration done");
    Ok(())
}

async fn app() -> Result<Router> {
    let app = by_axum::new();
    let conf = config::get();
    tracing::debug!("config: {:?}", conf);

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    if conf.migrate {
        tracing::info!("Running migration");
        migration(&pool).await?;
    }

    let app = app
        .nest(
            "/v1/contacts",
            v1::contacts::ContactController::new(pool.clone()).route()?,
        )
        .nest(
            "/v1/members",
            v1::members::MemberController::new(pool.clone()).route()?,
        )
        .nest("/v1/news", v1::news::NewsController::new(pool).route()?)
        .layer(middleware::from_fn(authorization_middleware));

    Ok(app)
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = app().await?;

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use std::time::SystemTime;

    use super::*;
    use rest_api::ApiService;

    pub struct TestContext {
        pub pool: sqlx::Pool<sqlx::Postgres>,
        pub app: Box<dyn ApiService>,
        pub now: i64,
        pub endpoint: String,
    }

    pub async fn setup() -> Result<TestContext> {
        let app = super::app().await?;

        let app = by_axum::into_api_adapter(app);
        let app = Box::new(app);
        rest_api::set_api_service(app.clone());

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let pool = if let DatabaseConfig::Postgres { url, pool_size } = config::get().database {
            PgPoolOptions::new()
                .max_connections(pool_size)
                .connect(url)
                .await?
        } else {
            panic!("Database is not initialized. Call init() first.");
        };

        Ok(TestContext {
            pool,
            app,
            now: now as i64,
            endpoint: format!("http://localhost:3000"),
        })
    }
}
