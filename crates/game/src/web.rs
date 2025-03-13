use wasm_bindgen::JsCast;

use game_logic::Config;

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

pub(crate) fn get_config() -> Config {
    Config {
        bot_count: get_value_from_select("bot-count")
            .unwrap_or("1".to_string())
            .parse::<usize>()
            .unwrap_or(1),
        obstacles: get_value_from_select("obstacles").unwrap_or("false".to_string()) == "true",
        variable_fish_value: get_value_from_select("variable-fish-value")
            .unwrap_or("false".to_string())
            == "true",
    }
}

fn get_value_from_select(id: &str) -> Option<String> {
    Some(
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let element = doc.get_element_by_id(id)?;
                Some(element.dyn_into::<web_sys::HtmlSelectElement>().ok()?)
            })?
            .value(),
    )
}
