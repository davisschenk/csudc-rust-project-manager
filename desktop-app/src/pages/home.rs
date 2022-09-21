use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(
    rsx!{
        Link {
            to: "",
            "Go to other page"
        }
    })
}
