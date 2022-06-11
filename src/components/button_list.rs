use yew::{function_component, html};
use crate::components::button::Button;

#[function_component(ButtonList)]
pub fn button_list() -> Html {
    html! {
      <>
        <div class="flex justify-center my-5">
        { for (0..5).map(|i| html! {
                <Button content={ format!("T{}", i) } />
        }) }
        </div>

        <div class="flex justify-center my-5">
        { for (0..5).map(|i| html! {
                <Button content={ format!("S{}", i) } />
        }) }
        </div>
      </>
    }
}
