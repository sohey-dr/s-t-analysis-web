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

fn main() {
    yew::start_app::<App>();
}
