use crate::Route;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct NavbarItemProps {
    to: Route,
    children: Element,
}

#[component]
pub fn NavbarItem(props: NavbarItemProps) -> Element {
    rsx! {
        Link {
            class: "block p-2 text-slate-600 hover:bg-blue-500 hover:text-blue-800 dark:hover:bg-blue-800 dark:hover:text-blue-500 rounded-lg cursor-pointer",
            active_class: "link-active",
            to: props.to,
            {props.children}
        }
    }
}
