mod pages;
mod components;
mod router;

use yew::prelude::*;
use yew_router::prelude::*;

use router::router::{Route, switch};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
