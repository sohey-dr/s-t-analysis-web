mod pages;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::top::Top;
use pages::recording::Recording;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Top,
    #[at("/recording")]
    Recording,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Top => html! { <Top /> },
        Route::Recording => html! { <Recording /> },
        Route::NotFound => html! { { "404" } },
    }
}


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
