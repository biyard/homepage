#![allow(non_snake_case)]

pub mod pages {
    pub mod home;
}

pub mod layouts {
    pub mod root_layout;
}

pub mod components {
    pub mod feature_button;
    pub mod filled_button;
    pub mod icon_button;
    pub mod icons;
    pub mod image_card;
    pub mod outlined_button;
}

pub mod routes;

pub mod prelude {
    pub use crate::components::icon_button::*;
    pub use crate::components::icons;
    pub use crate::components::outlined_button::*;
    pub use crate::layouts::root_layout::RootLayout;
    pub use crate::pages::home::Home;
}

pub mod apis;
pub mod models;

use dioxus::prelude::*;
use dioxus_logger::tracing::{self, Level};
use routes::Route;

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
    launch(App);
}

fn App() -> Element {
    rsx! {
        for href in vec![
            "https://fonts.googleapis.com",
            "https://fonts.gstatic.com",
        ] {
            head::Link {
                rel: "preconnect",
                href,
            }
        }
        for href in vec![
            "https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap",
            "https://fonts.googleapis.com/icon?family=Material+Icons",
            "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0",
        ] {
            head::Link {
                rel: "stylesheet",
                href,
            }
        }
        head::Link {
            rel: "icon",
            r#type: "image/x-icon",
            href: asset!("./assets/favicon.ico"),
        }
        head::Link {
            rel: "stylesheet",
            href: asset!("./assets/main.css")
        }
        load_tailwindcss {}
        Router::<Route> {}
    }
}

#[cfg(not(feature = "lambda"))]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {
        head::Script {
            src: "https://cdn.tailwindcss.com/3.4.5",
            ""
        }
    }
}

#[cfg(feature = "lambda")]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {
        head::Link {
            rel: "stylesheet",
            href: asset!("./assets/tailwind.css")
        }
    }
}
