#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[component]
pub fn IconButton(
    children: Element,
    onclick: Option<EventHandler<MouseEvent>>,
    disable_dots: Option<bool>,
) -> Element {
    rsx! {
        div {
            class: "relative flex cursor-pointer transition-all duration-300 ease-in-out p-[10px] rounded-[5px] bg-[radial-gradient(87.5%_87.5%_at_52.08%_25%,#4A9B7F_0%,#2B4584_100%)] hover:bg-[radial-gradient(87.5%_87.5%_at_52.08%_25%,#FF1B6B_0%,#2B4584_100%)]",
            onclick: move |evt| {
                tracing::debug!("icon button clicked");
                if let Some(onclick) = onclick {
                    onclick(evt);
                };
            },
            {children}
            if !disable_dots.unwrap_or(false) {
                for i in 0..10 {
                    Dot { i: i as f32 }
                    Dot { i: (i+1) as f32 / 1 as f32 }
                    Dot { i: (i+1) as f32 / 2 as f32 }
                }
            }

        }
    }
}

#[component]
pub fn Dot(i: f32) -> Element {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let rnd: f32 = rng.gen_range(0.00000001..0.9999999999);
    let r: f32 = rng.gen_range(0.00000001..0.9999999999);
    let a: f32 = rng.gen_range(0.00000001..0.9999999999);

    rsx! {
        i {
            style: format!("--i: {}; --rnd: {}; --r: {}; --a: {}", i, rnd, r, a),
            class: "spark",
        }
    }
}
