mod components;

use yew::prelude::*;

use components::act_style_button_list::ActStyleButtonList;

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| "未選択".to_string());

    let seconds = use_state(|| 5);
    let oninput = {
        let seconds = seconds.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.data();
            match value {
                Some(value) => {
                    seconds.set(value.parse::<i32>().unwrap());
                },
                None => {
                    seconds.set((*seconds).clone());
                }
            }
        })
    };

    html! {
        <>
            <div class="flex justify-center mt-3">
                <h1 class="text-3xl font-bold text-center">{ "選択されている要素" }</h1>
            </div>

            <div class="flex justify-center mt-3">
                <span class="text-2xl text-center">{ (*selected).clone() }</span>
            </div>

            <ActStyleButtonList selected={ selected } />

            <div class="flex justify-center mt-3">
                // TODO: 数字をinputから変更できるようにする
                <input
                    type="number"
                    name="seconds"
                    value={ (*seconds).clone().to_string() }
                    { oninput }
                    id="seconds"
                    class="text-center"
                />
                <act_style_button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                    { "秒でStart" }
                </act_style_button>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
