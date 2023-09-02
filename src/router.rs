mod about;
mod home;
mod not_found;

use about::About;
use home::Home;
use not_found::NotFound;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
