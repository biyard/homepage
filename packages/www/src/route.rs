use bdk::prelude::*;

use crate::pages::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(IndexLayout)]
            #[route("/")]
            IndexPage { lang: Language },
        #[end_layout]
    #[end_nest]

    #[redirect("/", || Route::IndexPage { lang: Language::En })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
