use crate::router::Route;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn Header(cx: Scope) -> Element {
    render!(Router::<Route> {})
}
