use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx!( div { style: "background: #112233; color: #fffeee;", "Home Page" } ))
}
