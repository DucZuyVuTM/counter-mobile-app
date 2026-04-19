#![allow(non_snake_case)]
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Logger for Android OS
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("COUNTER_APP"),
        );
        log::info!("Android logger is ready!");
    }

    // Logger for Web
    #[cfg(target_arch = "wasm32")]
    {
        wasm_logger::init(wasm_logger::Config::default());
        log::info!("Web logger is ready!");
    }

    // For running in Mobile/Desktop, we use launch
    // For running in Web, we use dioxus_web::launch
    launch(App);
}

#[component]
fn App() -> Element {
    use_effect(|| {
        spawn(async {
            let _ = document::eval(r#"
                new Promise(resolve => {
                    setTimeout(() => {
                        const counter = document.getElementById('counter');
                        if (counter) counter.style.visibility = 'visible';
                        resolve();
                    }, 300);
                })
            "#).await;
        });
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            id: "counter",
            style: "visibility: hidden;",
            Counter {}
        }
    }
}

#[component]
pub fn Counter() -> Element {
    let mut count = use_signal(|| 0);

    // Call JS for triggering animation
    let trigger_animation = move || {
        spawn(async {
            let js_code = r#"
                const counterEl = document.getElementById('counter-display');
                if (counterEl) {
                    // Remove class
                    counterEl.classList.remove('scale-animate');
                    
                    // Force reflow so that browser can realize that class is removed
                    void counterEl.offsetWidth;
                    
                    // Add class again to trigger animation
                    counterEl.classList.add('scale-animate');
                }
            "#;
            
            let _ = document::eval(js_code).await;
        });
    };

    let handle_increment = move |_| {
        count += 1;
        trigger_animation();
    };

    let handle_decrement = move |_| {
        count -= 1;
        trigger_animation();
    };

    let handle_reset = move |_| {
        count.set(0);
        trigger_animation();
    };

    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-gray-100 font-sans",
            h1 { class: "text-4xl font-bold text-gray-800 mb-4", "Basic Counter" }

            h2 {
                id: "counter-display",
                class: "text-6xl text-center font-mono text-blue-600 mb-8 max-w-full break-words",
                "{count}"
            }

            div { class: "flex flex-col gap-4 sm:flex-row",
                button {
                    class: "px-6 py-3 bg-green-500 hover:bg-green-600 text-white font-semibold rounded-lg shadow-md transition duration-200",
                    onclick: handle_increment,
                    "Increase"
                }

                button {
                    class: "px-6 py-3 bg-red-500 hover:bg-red-600 text-white font-semibold rounded-lg shadow-md transition duration-200",
                    onclick: handle_decrement,
                    "Decrease"
                }

                button {
                    class: "px-6 py-3 bg-gray-500 hover:bg-gray-600 text-white font-semibold rounded-lg shadow-md transition duration-200",
                    onclick: handle_reset,
                    "Reset"
                }
            }
        }
    }
}
