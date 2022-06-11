mod components;

use yew::prelude::*;
use components::button::Button;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Button />
    }
}

fn main() {
    yew::start_app::<App>();
}
