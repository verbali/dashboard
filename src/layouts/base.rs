use crate::components::layouts::{Footer, Header, Navbar};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BaseLayout() -> Element {
    rsx! {
        Header {}
        Navbar {}
        Outlet::<Route> {}
        Footer {}
    }
}
