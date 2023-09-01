#![allow(non_snake_case)]

mod layout;
mod router;

use layout::Layout;

use dioxus::prelude::*;

fn App(cx: Scope) -> Element {
    cx.render(rsx!(Layout {}))
}

fn main() {
    dioxus_web::launch(App);
}
