use yew::{function_component, html};

use crate::components::seconds_setter::SecondsSetter;

#[function_component(Top)]
pub fn top() -> Html {
    html! {
        <>
            <SecondsSetter />
        </>
    }
}
