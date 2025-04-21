use bdk::prelude::*;
use common::MemberSummary;

#[component]
pub fn MemberCard(member: MemberSummary, class: Option<String>) -> Element {
    rsx! {
        div { class: "group w-full h-full overflow-hidden min-w-300 {member.role.color()} rounded-[20px] overflow-hidden",
            div { class: "relative h-300 transition-all duration-1000 flex flex-col gap-[10px] items-center justify-center bg-cover",
                div { class: "absolute top-0 left-0 w-full py-44 flex flex-col items-center justify-start",
                    img { class: " object-cover z-1", src: member.image }
                }
                div { class: "member-card-bg z-2" }
                div { class: "absolute top-0 left-0 z-3 w-full h-full flex flex-col items-start justify-end rounded-[8px] p-24 transition-all duration-1000 group-hover:top-[100%]",
                    div { class: "text-lg/25 font-medium text-opacity-30 transition-all duration-1000",
                        {member.role.translate(&Language::En)}
                    }

                    div { class: "text-2xl/34 font-bold transition-all duration-1000",
                        "{member.name}"
                    }
                }

                div { class: "absolute transition-all duration-1000 top-[100%] left-0 w-full h-full group-hover:top-0 flex flex-col items-start justify-start p-[20px] z-4 bg-black/85",
                    div { class: "flex flex-col w-full",
                        div { class: "text-[24px] font-black", {member.role.translate(&Language::En)} }
                        div { class: "w-[68px] h-[5px] bg-white" }
                        div { class: "flex flex-row justify-between items-center w-full",
                            div { class: "text-[20px] font-black", "{member.name}" }
                            div { class: "flex flex-row gap-[4px] items-center justify-center",
                                a { href: format!("mailto:{}", member.email), Email {} }
                                if member.web.is_some() {
                                    a { href: member.web.clone().unwrap(), WebIcon {} }
                                }
                                if member.linkedin.is_some() {
                                    a { href: member.linkedin.clone().unwrap(), Linkedin {} }
                                }
                                if member.github.is_some() {
                                    a { href: member.github.clone().unwrap(), Github {} }
                                }
                            }
                        }
                    }

                    div { class: "text-[14px] font-regular", "{member.description}" }
                }
            }
        }
    }
}

#[component]
pub fn Email() -> Element {
    rsx! {
        svg {
            width: "20",
            height: "16",
            view_box: "0 0 20 16",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M3.5 0.5H16.5C17.2956 0.5 18.0587 0.816071 18.6213 1.37868C19.1839 1.94129 19.5 2.70435 19.5 3.5V12.5C19.5 13.2956 19.1839 14.0587 18.6213 14.6213C18.0587 15.1839 17.2956 15.5 16.5 15.5H3.5C2.70435 15.5 1.94129 15.1839 1.37868 14.6213C0.816071 14.0587 0.5 13.2956 0.5 12.5V3.5C0.5 2.70435 0.816071 1.94129 1.37868 1.37868C1.94129 0.816071 2.70435 0.5 3.5 0.5ZM3.5 1.5C3 1.5 2.56 1.67 2.22 1.97L10 7L17.78 1.97C17.44 1.67 17 1.5 16.5 1.5H3.5ZM10 8.21L1.63 2.78C1.55 3 1.5 3.25 1.5 3.5V12.5C1.5 13.0304 1.71071 13.5391 2.08579 13.9142C2.46086 14.2893 2.96957 14.5 3.5 14.5H16.5C17.0304 14.5 17.5391 14.2893 17.9142 13.9142C18.2893 13.5391 18.5 13.0304 18.5 12.5V3.5C18.5 3.25 18.45 3 18.37 2.78L10 8.21Z",
                fill: "white",
            }
        }
    }
}

#[component]
pub fn WebIcon() -> Element {
    rsx! {
        svg {
            width: "21",
            height: "20",
            view_box: "0 0 21 20",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M10.518 17.7077C14.7742 17.7077 18.2096 14.2556 18.2096 9.98268C18.2096 5.72643 14.7742 2.29102 10.518 2.29102M10.518 17.7077C6.24505 17.7077 2.79297 14.2556 2.79297 9.98268C2.79297 5.72643 6.24505 2.29102 10.518 2.29102M10.518 17.7077V2.29102M17.2326 13.7598H3.77505M2.83464 9.99935H18.168M17.1896 6.14518H10.5038H3.81797H10.518H17.1896Z",
                stroke: "white",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M10.507 17.7077C12.6354 17.7077 14.3529 14.2556 14.3529 9.98268C14.3529 5.72643 12.6354 2.29102 10.507 2.29102C8.37036 2.29102 6.64453 5.72643 6.64453 9.98268C6.64453 14.2556 8.37036 17.7077 10.507 17.7077Z",
                stroke: "white",
                stroke_linecap: "round",
                stroke_linejoin: "round",

            }
        }
    }
}

#[component]
pub fn Linkedin() -> Element {
    rsx! {
        svg {
            width: "21",
            height: "20",
            view_box: "0 0 21 20",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M11.0779 9.30849V9.28516L11.0625 9.30891L11.0779 9.30849Z",
                fill: "white",
            }
            path {
                d: "M7.01562 5.92188C7.18821 5.92188 7.32812 5.78196 7.32812 5.60938C7.32812 5.43679 7.18821 5.29688 7.01562 5.29688C6.84304 5.29688 6.70312 5.43679 6.70312 5.60938C6.70312 5.78196 6.84304 5.92188 7.01562 5.92188Z",
                fill: "white",
            }
            path {
                d: "M7.01562 7.05469V13.2972M14.2948 13.2972V9.29385C14.2948 8.9752 14.2321 8.65965 14.1102 8.36524C13.9883 8.07083 13.8096 7.80331 13.5843 7.57797C13.359 7.35263 13.0915 7.17387 12.7971 7.05192C12.5027 6.92996 12.1872 6.86719 11.8685 6.86719C11.5499 6.86713 11.2343 6.92986 10.9398 7.0518C10.6454 7.17373 10.3779 7.35247 10.1525 7.57782C9.92716 7.80317 9.74841 8.0707 9.62648 8.36514C9.50455 8.65959 9.44182 8.97516 9.44187 9.29385M9.44187 9.29385V13.2972M9.44187 9.29385V7.05469",
                stroke: "white",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M16.543 2.29102H4.45964C4.01761 2.29102 3.59368 2.46661 3.28112 2.77917C2.96856 3.09173 2.79297 3.51565 2.79297 3.95768V16.041C2.79297 16.483 2.96856 16.907 3.28112 17.2195C3.59368 17.5321 4.01761 17.7077 4.45964 17.7077H16.543C16.985 17.7077 17.4089 17.5321 17.7215 17.2195C18.034 16.907 18.2096 16.483 18.2096 16.041V3.95768C18.2096 3.51565 18.034 3.09173 17.7215 2.77917C17.4089 2.46661 16.985 2.29102 16.543 2.29102Z",
                stroke: "white",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn Github() -> Element {
    rsx! {
        svg {
            width: "19",
            height: "19",
            view_box: "0 0 19 19",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M9.49918 0.990236C7.47765 0.989197 5.52176 1.70779 3.98174 3.01735C2.44172 4.32691 1.41814 6.1419 1.0943 8.13732C0.770463 10.1327 1.1675 12.1783 2.21433 13.9077C3.26116 15.6371 4.88941 16.9373 6.80752 17.5757C7.23502 17.6548 7.38939 17.3936 7.38939 17.1798V15.735C5.01439 16.2496 4.52356 14.595 4.52356 14.595C4.36646 14.0715 4.02467 13.6228 3.56168 13.3323C2.78981 12.8059 3.62106 12.8138 3.62106 12.8138C3.89068 12.8515 4.14814 12.9503 4.37381 13.1026C4.59949 13.2548 4.78744 13.4566 4.92335 13.6925C5.68335 14.9948 6.90252 14.6188 7.40127 14.4011C7.4403 13.9686 7.63263 13.5642 7.94356 13.2611C6.05148 13.0473 4.06439 12.319 4.06439 9.05732C4.05226 8.21124 4.36704 7.39311 4.94314 6.77336C4.68124 6.03968 4.711 5.23345 5.02627 4.52107C5.02627 4.52107 5.73877 4.29149 7.36564 5.3919C8.75994 5.0119 10.2305 5.0119 11.6248 5.3919C13.2517 4.29149 13.9642 4.52107 13.9642 4.52107C14.2771 5.234 14.3068 6.03931 14.0473 6.77336C14.6227 7.39329 14.9361 8.21164 14.9221 9.05732C14.9221 12.3269 12.9429 13.0434 11.039 13.2571C11.2433 13.4641 11.4006 13.7126 11.5003 13.9858C11.6001 14.2591 11.6398 14.5505 11.6169 14.8404V17.1759C11.6169 17.4569 11.7713 17.6667 12.2027 17.5717C14.1189 16.9309 15.7445 15.6293 16.7889 13.8996C17.8333 12.17 18.2282 10.1253 17.9031 8.13115C17.5779 6.13702 16.5539 4.32361 15.0143 3.01532C13.4746 1.70703 11.5196 0.989195 9.49918 0.990236Z",
                stroke: "white",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
