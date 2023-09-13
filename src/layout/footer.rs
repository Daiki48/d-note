use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx!(
        section {
            div { h1 { "Footerです" } }
        }
    ))
}
