use dioxus::prelude::*;

use crate::backend;

#[component]
pub fn Favorites() -> Element {
    let favorites = use_server_future(backend::list_dogs)?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id , url) in favorites().unwrap().unwrap() {
                    div { key: "{id}", class: "favorite-dog",
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}
