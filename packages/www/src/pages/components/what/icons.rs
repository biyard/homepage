use bdk::prelude::*;

#[component]
pub fn Pratical(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = 35)] size: i64,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "33",
            view_box: "0 0 32 33",
            width: "32",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M15.9993 2.76343L2.66602 16.0967V2.76343H15.9993Z",
                fill: "#00D190",
            }
            path {
                d: "M2.66602 16.0967L15.9993 29.43H2.66602V16.0967Z",
                fill: "#00D190",
            }
            path {
                d: "M16.0007 2.76343L29.334 16.0967V2.76343H16.0007Z",
                fill: "#00D190",
            }
            path {
                d: "M29.334 16.0967L16.0007 29.43H29.334V16.0967Z",
                fill: "#00D190",
            }
        }
    }
}

#[component]
pub fn PeopleCentric(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = 35)] size: i64,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "33",
            view_box: "0 0 32 33",
            width: "32",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M29.3327 16.0968V2.76343H15.9993H2.66602V16.0968L15.9994 16.0968L15.9993 29.4301L29.3327 16.0968ZM29.3327 16.0968L29.3327 29.4301L15.9993 29.4301L2.66602 16.0968L15.9993 2.76343L29.3327 16.0968Z",
                fill: "#00D190",
                fill_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn Scalable(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = 35)] size: i64,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "33",
            view_box: "0 0 32 33",
            width: "32",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M15.9993 2.76343L2.66602 16.0967V2.76343H15.9993Z",
                fill: "#00D190",
            }
            path {
                d: "M2.66602 16.0967L15.9993 29.43V16.0967L2.66602 16.0967Z",
                fill: "#00D190",
            }
            path {
                d: "M16.0007 2.76343L29.334 16.0967L16.0007 16.0968V2.76343Z",
                fill: "#00D190",
            }
            path {
                d: "M29.334 16.0967L16.0007 29.43H29.334V16.0967Z",
                fill: "#00D190",
            }
        }
    }
}
