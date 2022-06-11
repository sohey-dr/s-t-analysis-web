mod components;

use yew::prelude::*;

use components::button_list::ButtonList;

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| "".to_string());
    let onclick = {
        let selected = selected.clone();
        Callback::from(move |_| selected.set("Clicked!".to_string()))
    };


    html! {
        <>
            <div class="flex justify-center">
                <h1>{ (*selected).clone() }</h1>
            </div>

            <ButtonList onclick={ onclick } />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
