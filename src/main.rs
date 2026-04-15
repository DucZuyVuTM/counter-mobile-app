#![allow(non_snake_case)]
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // For running in Mobile/Desktop, we use launch
    // For running in Web, we use dioxus_web::launch
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Counter {}
    }
}

#[component]
pub fn Counter() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-gray-100 font-sans",
            h1 {
                class: "text-4xl font-bold text-gray-800 mb-4",
                "Basic Counter"
            }

            h2 {
                class: "text-6xl font-mono text-blue-600 mb-8",
                "{count}"
            }

            div { class: "flex gap-4",
                button {
                    class: "px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-semibold rounded-lg shadow-md transition duration-200",
                    onclick: move |_| count += 1,
                    "Increase"
                }

                button {
                    class: "px-6 py-3 bg-red-500 hover:bg-red-600 text-white font-semibold rounded-lg shadow-md transition duration-200",
                    onclick: move |_| count -= 1,
                    "Decrease"
                }

                button {
                    class: "px-6 py-3 bg-gray-500 hover:bg-gray-600 text-white font-semibold rounded-lg shadow-md transition duration-200",
                    onclick: move |_| count.set(0),
                    "Reset"
                }
            }
        }
    }
}
