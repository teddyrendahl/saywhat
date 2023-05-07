use yew::prelude::*;

use crate::words;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};

/// Get the value out of the text from the input event
fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();

    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

/// Get the first N words that are within edit_distance of the provided value
fn get_matches(s: &str, max_edit_distance: usize, words: usize) -> Vec<String> {
    words::WORDS
        .iter()
        .filter_map(|w| {
            let d = saywhat::edit_distance(
                &s.chars().collect::<Vec<char>>(),
                &w.chars().collect::<Vec<char>>(),
                1,
                false,
            );
            if d <= max_edit_distance {
                web_sys::console::log(&wasm_bindgen::JsValue::from_str(w).into());
                Some(w.to_string())
            } else {
                None
            }
        })
        .take(words)
        .collect()
}

/// Widget that shows the N closest values based on edit_distance
#[function_component(TextBox)]
pub fn text_box() -> Html {
    let guess_handle = use_state(Vec::new);

    let on_input_guesses = guess_handle.clone();
    let on_input = Callback::from(move |input_event: InputEvent| {
        let entry = get_value_from_input_event(input_event);
        on_input_guesses.set(get_matches(&entry, 1, 5));
    });

    html! {
          <div>
            <input type="search" list="options" name="browser" id="browser" oninput={on_input}/>
           {guess_handle.iter().cloned().map(|guess|
                  html!{<p>{guess}</p>}
              ).collect::<Html>()}
          </div>
    }
}
