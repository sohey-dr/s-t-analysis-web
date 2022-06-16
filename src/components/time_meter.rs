use yew::{function_component, html, Properties, Callback, use_state};
use yew_hooks::{use_interval};

#[function_component(TimeMeter)]
pub fn time_meter() -> Html {
    let seconds = use_state(|| 0);

    {
      let seconds = seconds.clone();
      use_interval(
        move || {
          seconds.set(seconds.get() + 1);
        },
        1000,
      );
    }
    html! {
        <>
            <div>{ *seconds }</div>
        </>
    }
}
