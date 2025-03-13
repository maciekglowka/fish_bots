use wasm_bindgen::JsCast;

// pub(crate) fn get_bot_code() -> String {
//     let textarea = web_sys::window()
//         .and_then(|win| win.document())
//         .and_then(|doc| {
//             let element = doc.get_element_by_id("bot-code")?;
//             Some(
//                 element
//                     .dyn_into::<web_sys::HtmlDivElement>()
//                     .map_err(|_| ())
//                     .expect("Html element is not a div"),
//             )
//         })
//         .expect("Can't find bot-code!");
//     textarea.inner_text()
// }
pub(crate) fn get_bot_code() -> String {
    let textarea = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let element = doc.get_element_by_id("bot-code")?;
            Some(
                element
                    .dyn_into::<web_sys::HtmlTextAreaElement>()
                    .map_err(|_| ())
                    .expect("Html element is not a textarea"),
            )
        })
        .expect("Can't find bot-code!");
    textarea.value()
}

pub(crate) fn write_output(s: String) {
    let textarea = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let element = doc.get_element_by_id("console")?;
            Some(
                element
                    .dyn_into::<web_sys::HtmlDivElement>()
                    .map_err(|_| ())
                    .expect("Html element is not a div"),
            )
        })
        .expect("Can't find bot-code!");
    let mut current = textarea.inner_text();
    textarea.set_inner_text(&format!("{}{}\n", current, &s));
}

pub(crate) fn get_config() {
    let input = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let element = doc.get_element_by_id("bot-count")?;
            Some(
                element
                    .dyn_into::<web_sys::HtmlSelectElement>()
                    .map_err(|_| ())
                    .expect("Html element is not a select"),
            )
        })
        .expect("Can't find bot-count!");
    rogalik::engine::log::info!("Got config {:?}", input.value());
}
