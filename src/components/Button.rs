use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub content: String,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
      <button class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow">
        { &props.content }
      </button>
    }
}
