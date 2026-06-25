use dioxus::prelude::*;

use crate::backend::save_dog;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    let skip = move |_evt| {
        img_src.restart();
    };
    let save = move |_| async move {
        let current = img_src.cloned().unwrap();
        img_src.restart();
        _ = save_dog(current).await;
    };
    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}
