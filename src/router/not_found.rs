use crate::router::Route;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[inline_props]
pub fn NotFound(cx: Scope, segments: Vec<String>) -> Element {
    cx.render(rsx!(
        div { class: "text-xl bg-red-400", "ページが見つかりません" }
        Link { to: Route::Home {}, "Homeへ戻る" }
        pre { color: "red", "log:\nattemped to navigate to: {segments:?}" }
    ))
}
