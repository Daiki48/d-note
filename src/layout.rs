mod footer;
mod header;
mod hero;

use footer::Footer;
use header::Header;
// use hero::Hero;
use crate::router::Hero;

use dioxus::prelude::*;

pub fn Layout(cx: Scope) -> Element {
    cx.render(rsx!(
        header { Header {} }
        main { Hero {} }
        footer { Footer {} }
    ))
}
