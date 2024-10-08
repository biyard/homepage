#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::icons;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ButtonShape {
    #[serde(rename = "rounded-rect")]
    RoundedRect,
    #[serde(rename = "circle")]
    Circle,
}

impl std::fmt::Display for ButtonShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonShape::RoundedRect => write!(f, "rounded-rect"),
            ButtonShape::Circle => write!(f, "circle"),
        }
    }
}

impl std::str::FromStr for ButtonShape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rounded-rect" => Ok(ButtonShape::RoundedRect),
            "circle" => Ok(ButtonShape::Circle),
            _ => Err(format!("Invalid type: {}", s)),
        }
    }
}

impl ButtonShape {
    pub fn to_string(&self) -> String {
        match self {
            ButtonShape::RoundedRect => "rounded-rect".to_string(),
            ButtonShape::Circle => "circle".to_string(),
        }
    }
}

#[component]
pub fn OutlinedButton(
    children: Element,
    onclick: Option<EventHandler<MouseEvent>>,
    shape: Option<ButtonShape>,
) -> Element {
    let mut is_loading = use_signal(|| false);
    let shape = shape.unwrap_or(ButtonShape::RoundedRect);
    let class = format!("border border-[#ffffff]/30 text-[13px] font-regular text-white hover:bg-white hover:text-[#03AB79] hover:border-white cursor-pointer transition-all duration-300 ease-in-out flex justify-center items-center gap-[5px] active:bg-[#03AB79] active:text-white active:border-[#03AB79] {}",  match shape {
        ButtonShape::RoundedRect => "rounded-[10px] px-[15px] py-[10px]",
        ButtonShape::Circle => "rounded-full p-[6.37px]",
    });

    rsx! {
        div {
            class,
            onclick: move |evt| {
                if let Some(onclick) = onclick {
                    is_loading.set(true);
                    onclick.call(evt);
                    is_loading.set(false);
                }
            },
            if is_loading() {
                icons::spin { }
            } else {
                {children}
            }
        }
    }
}
