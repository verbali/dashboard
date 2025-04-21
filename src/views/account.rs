use dioxus::prelude::*;

#[component]
pub fn Account() -> Element {
    rsx! {
        div {
            class: "max-w-6xl lg:w-6xl lg:mx-auto p-4",
            "Account"
        }
    }
}
