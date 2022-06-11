use yew::{function_component, html, Properties};

use crate::components::button::Button;

#[derive(Properties, PartialEq)]
pub struct ButtonListProps {
    pub onclick: yew::Callback<yew::MouseEvent>,
}

#[function_component(ButtonList)]
pub fn button_list(props: &ButtonListProps) -> Html {
    html! {
      <>
        <div class="flex justify-center my-5">
        { for (0..5).map(|i| html! {
                <Button content={ format!("T{}", i) } onclick={ (*props).onclick.clone() } />
        }) }
        </div>

        <div class="flex justify-center my-5">
        { for (0..5).map(|i| html! {
                <Button content={ format!("S{}", i) } onclick={ (*props).onclick.clone() } />
        }) }
        </div>
      </>
    }
}
