use crate::router::Route;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "text-xl bg-red-400", "Home Page" }
        Link { to: Route::About {}, "Aboutページへ移動" }
    ))
}
