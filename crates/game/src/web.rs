use wasm_bindgen::JsCast;

pub(crate) fn get_bot_code() -> String {
    let textarea = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let element = doc.get_element_by_id("bot-code")?;
            Some(
                element
                    .dyn_into::<web_sys::HtmlDivElement>()
                    .map_err(|_| ())
                    .expect("Html element is not a div"),
            )
        })
        .expect("Can't find bot-code!");
    textarea.inner_text()
}
