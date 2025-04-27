use crate::Route;
use dioxus::prelude::*;
use verbali_design_system::components::{icons::AccountIcon, logos::FullLogo};

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "max-w-6xl lg:w-6xl lg:mx-auto flex flex-row justify-between items-center p-4",
            Link {
                to: Route::Home {},
                FullLogo {}
            }
            Link {
                class: "flex gap-2",
                to: Route::Account {},
                AccountIcon {
                    size: "24px"
                }
                "Account"
            }
        }
    }
}
