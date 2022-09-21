
use dioxus::prelude::*;

pub fn NotFound(cx: Scope) -> Element {
    cx.render(
    rsx!{
        h1 {
            "Not Found"
        }
        Link {
            to: "/home",
            "Go Home!"
        }
    })
}
