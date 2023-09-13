use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx!(
        section { style: "display: flex; justify-content: space-between; padding: 6px;",
            div { style: "font-size: 2em;", h1 { "Dnote" } }
        }
    ))
}
