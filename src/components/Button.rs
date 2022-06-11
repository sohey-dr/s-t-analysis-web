use yew::{function_component, html};

#[function_component(Button)]
pub fn button() -> Html {
    html! {
      <button class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow">
        { "Button" }
      </button>
    }
}
