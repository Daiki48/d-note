mod header;

use header::Header;

use dioxus::prelude::*;

pub fn Layout(cx: Scope) -> Element {
    cx.render(rsx!(Header {}))
}
