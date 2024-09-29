#![allow(non_snake_case)]

pub mod pages {
    pub mod home;
}

pub mod layouts {
    pub mod root_layout;
}

pub mod services {
    pub mod popup_service;
}

pub mod components {
    pub mod download_popup;
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
use services::popup_service::PopupService;

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
    dioxus_aws::launch(App);
}

fn App() -> Element {
    PopupService::init();

    rsx! {
        head::Meta {
            name: "description",
            content: "Biyard is a leading blockchain technology company focused on building decentralized solutions that drive innovation and transparency. From secure digital content rights protection to enhancing the transparency, trust, and efficiency of public polls and surveys, we empower governments, enterprises, and developers to unlock the full potential of Web3. Our flagship platform, d.AGIT, pioneers new ways to safeguard and manage digital assets with trust and security. At Biyard, we’re shaping the future of a decentralized digital economy."
        }
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
        head::Link {
            rel: "stylesheet",
            href: asset!("./assets/tailwind.css")
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
    rsx! {}
}
