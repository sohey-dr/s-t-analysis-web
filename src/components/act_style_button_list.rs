use yew::{function_component, html, Properties};

use crate::components::act_style_button::ActStyleButton;

#[derive(Properties, PartialEq)]
pub struct ActStyleButtonListProps {
    pub selected: yew::UseStateHandle<std::string::String>,
}

#[function_component(ActStyleButtonList)]
pub fn act_style_button_list(props: &ActStyleButtonListProps) -> Html {
    html! {
      <>
        <div class="flex justify-center my-5">
        { for (1..6).map(|i| html! {
                <ActStyleButton content={ format!("T{}", i) } selected={ (*props).selected.clone() } />
        }) }
        </div>

        <div class="flex justify-center my-5">
        { for (1..6).map(|i| html! {
                <ActStyleButton content={ format!("S{}", i) } selected={ (*props).selected.clone() } />
        }) }
        </div>
      </>
    }
}
