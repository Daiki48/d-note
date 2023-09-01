mod about;
mod home;

use about::About;
use home::Home;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
}
