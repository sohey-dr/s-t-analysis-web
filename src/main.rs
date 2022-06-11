mod components;

use yew::prelude::*;
use components::button::Button;

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
            <button
                class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
                { onclick }
            >
                { "Click me" }
            </button>

            // TODO: forなどで回して、Buttonを複数表示する
            <div class="flex justify-center my-5">
                <Button content="S1" />
                <Button content="S2" />
                <Button content="S3" />
                <Button content="S4" />
            </div>
            <div class="flex justify-center my-5">
                <Button content="T1" />
                <Button content="T2" />
                <Button content="T3" />
                <Button content="T4" />
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
