use yew::{function_component, html, use_state};

use crate::components::seconds_setter::SecondsSetter;

#[function_component(Top)]
pub fn top() -> Html {
    let seconds = use_state(|| 5);

    html! {
        <>
            <SecondsSetter seconds={ seconds } />
        </>
    }
}
