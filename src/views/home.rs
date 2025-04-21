use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "max-w-6xl lg:w-6xl lg:mx-auto p-4",
            "Home"
        }
    }
}
