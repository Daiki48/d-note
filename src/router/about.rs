use crate::router::Route;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "text-xl bg-orange-400", "About Page" }
        Link { to: Route::Home {}, "Homeページへ移動" }
    ))
}
