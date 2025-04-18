use bdk::prelude::*;

use crate::route::Route;

#[component]
pub fn NotFoundPage(route: Vec<String>) -> Element {
    let nav = use_navigator();

    use_effect(move || {
        nav.replace(Route::IndexPage { lang: Language::En });
    });

    rsx! {}
}
