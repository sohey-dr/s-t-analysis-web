use yew::{function_component, html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub content: String,
    pub selected: yew::UseStateHandle<std::string::String>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let content = props.content.clone();
    let onclick = {
        let selected = props.selected.clone();
        Callback::from(move |_| selected.set(content.clone()))
    };

    html! {
      <button
        class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
        { onclick }
      >
        { &props.content }
      </button>
    }
}
