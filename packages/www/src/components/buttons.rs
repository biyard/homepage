use bdk::prelude::*;

#[component]
pub fn SecondaryRoundedButton(children: Element, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "bg-white text-bg font-semibold text-base py-15 px-40 rounded-full hover:bg-white/80 whitespace-nowrap",
            onclick: move |_| onclick(()),
            {children}
        }
    }
}

#[component]
pub fn SecondaryButton(
    #[props(default = "".to_string())] class: String,
    children: Element,
    onclick: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "bg-white text-bg font-semibold text-base py-15 px-40 rounded-[4px] hover:bg-white/80 {class}",
            onclick: move |_| onclick(()),
            {children}
        }
    }
}

#[component]
pub fn PrimaryButton(children: Element, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "bg-primary text-bg font-semibold text-base py-15 px-40 rounded-[4px] hover:bg-[linear-gradient(0deg,_#00E6A5,_#00E6A5),_linear-gradient(0deg,_rgba(0,0,0,0.2),_rgba(0,0,0,0.2))]",
            onclick: move |_| onclick(()),
            {children}
        }
    }
}

#[component]
pub fn OutlinedButton(
    #[props(default = "".to_string())] class: String,
    children: Element,
    onclick: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "text-white font-semibold text-base border border-white py-15 px-40 rounded-[4px] hover:bg-white/20 {class}",
            onclick: move |_| onclick(()),
            {children}
        }
    }
}
