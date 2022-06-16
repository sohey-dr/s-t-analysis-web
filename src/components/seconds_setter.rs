use yew::{function_component, html, Callback, InputEvent, use_state};
use yew_router::prelude::*;

use crate::router::router::Route;
use crate::pages::recording;

#[function_component(SecondsSetter)]
pub fn seconds_setter() -> Html {
    let seconds = use_state(|| 0);

    let oninput = {
        let seconds = seconds.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.data();
            match value {
                Some(value) => {
                  seconds.set(value.parse::<i32>().unwrap_or(0));
                },
                None => {
                  seconds.set(1);
                }
            }
        })
    };

    let navigator = use_history().unwrap();
    let onclick = {
        let seconds = seconds.clone();
        let query = recording::QueryParams {
            seconds: seconds.to_string(),
        };

        Callback::once(move |_| {
          navigator.push_with_query(Route::Recording, query).unwrap();
        })
    };

    html! {
      <>
        <div class="flex justify-center mt-3">
            // TODO: 数字をinputから変更できるようにする
            <input
                type="text"
                name="seconds"
                value={ (*seconds).clone().to_string() }
                { oninput }
                id="seconds"
                class="text-center"
            />
            <button
              class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
              { onclick }
            >
                { "秒でStart" }
            </button>
        </div>
      </>
    }
}
