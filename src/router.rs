mod about;
mod home;
mod not_found;

use about::About;
use home::Home;
use not_found::NotFound;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Hero)]
      #[route("/")]
      Home {},
      #[route("/about")]
      About {},
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[inline_props]
pub fn Hero(cx: Scope) -> Element {
    cx.render(rsx!(
        section {
            div { style: "color: red;", h1 { "Hero" } }
        }
    ))
}
