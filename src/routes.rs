use crate::layouts::root_layout::RootLayout;
use crate::prelude::*;
use dioxus::prelude::*;

#[derive(Routable, Clone, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(RootLayout)]
        #[route("/")]
        Home {},
}
