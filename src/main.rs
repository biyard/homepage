use bdk::prelude::{
    by_components::{effects::HoverEffects, responsive::Responsive},
    *,
};

pub mod assets;
pub mod components;
pub mod pages;
pub mod route;

pub use assets::*;
pub use components::*;

use dioxus_logger::tracing::{self, Level};
use route::Route;

fn main() {
    dioxus_logger::init(match option_env!("LOG_LEVEL") {
        Some("trace") => Level::TRACE,
        Some("debug") => Level::DEBUG,
        Some("info") => Level::INFO,
        Some("warn") => Level::WARN,
        Some("error") => Level::ERROR,
        _ => Level::INFO,
    })
    .expect("failed to init logger");

    tracing::info!("starting app");
    dioxus_aws::launch(app);
}

fn app() -> Element {
    let css = include_str!("../public/theme.css");

    rsx! {
        btracing::ToastTracing {}
        HoverEffects {}

        document::Link { href: asset!("/public/logos/favicon.ico"), rel: "shortcut icon" }
        document::Link {
            href: asset!("/public/logos/apple-touch-icon.png"),
            rel: "apple-touch-icon",
            sizes: "180x180",
        }

        document::Link { href: "https://fonts.googleapis.com", rel: "preconnect" }
        document::Link {
            crossorigin: "false",
            href: "https://fonts.gstatic.com",
            rel: "preconnect",
        }
        document::Style { href: "https://fonts.googleapis.com/css2?family=Noto+Sans+KR:wght@100..900&family=Outfit:wght@100..900&display=swap" }
        document::Style { href: asset!("/public/main.css") }
        document::Style { href: asset!("/public/tailwind.css") }

        document::Script { src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" }
        document::Script {
            src: "https://unpkg.com/@dotlottie/player-component@2.7.12/dist/dotlottie-player.mjs",
            r#type: "module",
        }
        document::Style { r#type: "text/tailwindcss", {css} }

        Responsive { tablet: 900.0, Router::<Route> {} }
    }
}
