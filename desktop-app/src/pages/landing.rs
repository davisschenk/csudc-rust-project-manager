use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaFolder;
use dioxus_free_icons::Icon;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use crate::types::DbUrl;

pub fn Landing(cx: Scope) -> Element {
    let url = cx.use_hook(|_| cx.consume_context::<DbUrl>());
    cx.render(
    rsx!{
        div {
            class: "jumbotron d-flex align-items-center min-vh-100",
            div {
                class: "container text-center",
                label {
                    class: "form-label",
                    r#for: "customFile",
                    "Select a directory for the CSUDC file manager"
                }
                div {
                    class: "row",
                    div {
                        class: "col-2",
                        button {
                            class: "form-control",
                            r#type: "file",
                            prevent_default: "true",
                            id: "customFile",
                            onclick: move |_| {
                                let path = FileDialog::new()
                                    .set_location("~/Desktop")
                                    .show_open_single_dir()
                                    .unwrap();


                                match &path {
                                    Some(p) => {
                                        println!("{:?}", path);
                                        url = DbUrl(Some(path.unwrap().to_str().unwrap().into()));
                                    },
                                    None => println!("No Path")
                                }
                            },
                            "Browse Files"
                        }
                    }
                    div {
                        class: "col-10",
                        "{url:?}"
                    }
                }
            }
        }
    })
}
