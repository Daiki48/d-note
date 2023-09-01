#![allow(non_snake_case)]

mod router;

use router::Route;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn App(cx: Scope) -> Element {
    render!(Router::<Route> {})
}

fn main() {
    dioxus_web::launch(App);
}
