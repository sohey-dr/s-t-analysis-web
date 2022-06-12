mod components;

use yew::prelude::*;

use components::button_list::ButtonList;

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| "未選択".to_string());

    html! {
        <>
            <div class="flex justify-center">
                <h1 class="text-3xl font-bold text-center">{ "選択されている要素" }</h1>
            </div>

            <div class="flex justify-center mt-3">
                <span class="text-2xl text-center">{ (*selected).clone() }</span>
            </div>

            <ButtonList selected={ selected } />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
