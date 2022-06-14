mod components;

use yew::prelude::*;

use components::button_list::ButtonList;

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| "未選択".to_string());

    html! {
        <>
            <div class="flex justify-center mt-3">
                <h1 class="text-3xl font-bold text-center">{ "選択されている要素" }</h1>
            </div>

            <div class="flex justify-center mt-3">
                <span class="text-2xl text-center">{ (*selected).clone() }</span>
            </div>

            // TODO: ActStyleButtonなど適切な名前に変更する
            <ButtonList selected={ selected } />

            <div class="flex justify-center mt-3">
                <input type="number" name="seconds" value="10" />
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                    { "秒でStart" }
                </button>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
