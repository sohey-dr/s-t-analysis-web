mod components;

use yew::prelude::*;

use components::act_style_button_list::ActStyleButtonList;
use components::seconds_setter::SecondsSetter;

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| "未選択".to_string());

    let seconds = use_state(|| 5);
    html! {
        <>
            <div class="flex justify-center mt-3">
                <h1 class="text-3xl font-bold text-center">{ "選択されている要素" }</h1>
            </div>

            <div class="flex justify-center mt-3">
                <span class="text-2xl text-center">{ (*selected).clone() }</span>
            </div>

            <ActStyleButtonList selected={ selected } />

            <SecondsSetter seconds={ seconds } />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
