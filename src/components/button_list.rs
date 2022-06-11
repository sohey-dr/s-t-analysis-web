use yew::{function_component, html, Properties};

use crate::components::button::Button;

#[derive(Properties, PartialEq)]
pub struct ButtonListProps {
    pub selected: yew::UseStateHandle<std::string::String>,
}

#[function_component(ButtonList)]
pub fn button_list(props: &ButtonListProps) -> Html {
    html! {
      <>
        <div class="flex justify-center my-5">
        { for (0..5).map(|i| html! {
                <Button content={ format!("T{}", i) } selected={ (*props).selected.clone() } />
        }) }
        </div>

        <div class="flex justify-center my-5">
        { for (0..5).map(|i| html! {
                <Button content={ format!("S{}", i) } selected={ (*props).selected.clone() } />
        }) }
        </div>
      </>
    }
}
