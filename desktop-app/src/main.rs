mod pages;
mod types;

use dioxus::prelude::*;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use crate::pages::{Home, NotFound, Landing};
use crate::types::DbUrl;

fn main() {
    dioxus::desktop::launch(app);
}




fn app(cx: Scope) -> Element {
    cx.use_hook(|_| {
        cx.provide_context(DbUrl(None));
    });

    cx.render(rsx!{
        link {
            href: "https://cdn.jsdelivr.net/npm/bootstrap@5.2.0-beta1/dist/css/bootstrap.min.css",
            rel: "stylesheet",
            integrity: "sha384-0evHe/X+R7YkIZDRvuzKMRqM+OrBnVFBL6DOitfPri4tjfHxaWutUpFmBp4vmVor",
            crossorigin: "anonymous"
        }
        script {
            src: "https://cdn.jsdelivr.net/npm/bootstrap@5.0.0-beta1/dist/js/bootstrap.bundle.min.js",
            integrity: "sha384-ygbV9kiqUc6oa4msXn9868pTtWMgiQaeYH7/t7LECLbyPA2x65Kgf80OJFdroafW",
            crossorigin:"anonymous"
        }

        Router {
            Route {to: "/landing", Landing {}},
            Route {to: "/home", Home {}},
            Redirect {from: "", to: "/landing"}
        }
    })
}
