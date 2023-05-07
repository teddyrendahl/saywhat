mod text_box;
mod words;

use crate::text_box::TextBox;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <TextBox />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
