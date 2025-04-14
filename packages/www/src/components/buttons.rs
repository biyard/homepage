use bdk::prelude::*;

#[component]
pub fn SecondaryButton(children: Element, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "bg-white text-bg font-semibold test-base py-20 px-40 rounded-[4px] hover:bg-[linear-gradient(0deg,_#FFFFFF,_#FFFFFF),_linear-gradient(0deg,_rgba(0,0,0,0.2),_rgba(0,0,0,0.2))]",
            onclick: move |_| onclick(()),
            {children}
        }
    }
}

#[component]
pub fn PrimaryButton(children: Element, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "bg-primary text-bg font-semibold test-base py-20 px-40 rounded-[4px] hover:bg-[linear-gradient(0deg,_#00E6A5,_#00E6A5),_linear-gradient(0deg,_rgba(0,0,0,0.2),_rgba(0,0,0,0.2))]",
            onclick: move |_| onclick(()),
            {children}
        }
    }
}

#[component]
pub fn OutlinedButton(children: Element, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "text-white font-semibold test-base border border-white py-20 px-40 rounded-[4px] hover:bg-white/20",
            onclick: move |_| onclick(()),
            {children}
        }
    }
}
