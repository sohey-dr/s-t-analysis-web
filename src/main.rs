mod components;

use yew::prelude::*;

use components::button_list::ButtonList;

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| "未選択".to_string());

    html! {
        <>
            <div class="flex justify-center">
                <h1>{ (*selected).clone() }</h1>
            </div>

            <ButtonList selected={ selected } />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
