use yew::{function_component, html, Properties, Callback, InputEvent};

#[derive(Properties, PartialEq)]
pub struct SecondsSetterProps {
    pub seconds: yew::UseStateHandle<i32>,
}

#[function_component(SecondsSetter)]
pub fn seconds_setter(props: &SecondsSetterProps) -> Html {
    let oninput = {
        let seconds = (*props).seconds.clone();
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
                value={ (*props).seconds.clone().to_string() }
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
