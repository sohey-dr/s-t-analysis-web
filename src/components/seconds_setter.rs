use yew::{function_component, html, Callback, InputEvent, use_state};


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
    let onclick = {
        Callback::from(move |_| {
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
