use crate::router::Route;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn Hero(cx: Scope) -> Element {
    cx.render(rsx!(
        section {
            div { style: "color: red;", h1 { "Hero" } }
            nav {
                ul {
                    li {
                        Link { to: Route::Home {}, "Home" }
                    }
                    li {
                        Link { to: Route::About {}, "About" }
                    }
                }
            }
        }
    ))
}
