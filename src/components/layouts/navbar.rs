use crate::components::layouts::NavbarItem;
use crate::Route;
use dioxus::prelude::*;
use verbali_design_system::components::icons::{FormIcon, HomeIcon};

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "fixed flex flex-col gap-2 top-[50%] -translate-y-[50%] ml-4 p-2 bg-slate-300 dark:bg-slate-800 rounded-lg",

            NavbarItem {
                to: Route::Home {},
                HomeIcon {
                    size: "24px"
                }
            }

            NavbarItem {
                to: Route::Forms {},
                FormIcon {
                    size: "24px"
                }
            }
        }
    }
}
