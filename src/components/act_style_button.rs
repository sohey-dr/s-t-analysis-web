use yew::{function_component, html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct ActStyleButtonProps {
    pub content: String,
    pub selected: yew::UseStateHandle<std::string::String>,
}

#[function_component(ActStyleButton)]
pub fn act_style_button(props: &ActStyleButtonProps) -> Html {
    let onclick = {
        let selected = props.selected.clone();
        let content  = props.content.clone();
        Callback::from(move |_| selected.set(content.clone()))
    };

    html! {
      <act_style_button
        class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
        { onclick }
      >
        { &props.content }
      </act_style_button>
    }
}
