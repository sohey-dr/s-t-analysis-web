use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::top::Top;
use crate::pages::recording::Recording;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/s-t-analysis-web")]
    Top,
    #[at("/s-t-analysis-web/recording")]
    Recording,
    #[not_found]
    #[at("/s-t-analysis-web/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Top => html! { <Top /> },
        Route::Recording => html! { <Recording /> },
        Route::NotFound => html! { { "404" } },
    }
}

