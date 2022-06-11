mod components;

use yew::prelude::*;
use components::button::Button;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Button content="S1" />
            <Button content="S2" />
            <Button content="S3" />
            <Button content="S4" />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
