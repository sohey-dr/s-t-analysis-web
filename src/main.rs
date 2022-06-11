mod components;

use yew::prelude::*;
use components::button::Button;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            // TODO: forなどで回して、Buttonを複数表示する
            <div class="flex justify-center my-5">
                <Button content="S1" />
                <Button content="S2" />
                <Button content="S3" />
                <Button content="S4" />
            </div>
            <div class="flex justify-center my-5">
                <Button content="T1" />
                <Button content="T2" />
                <Button content="T3" />
                <Button content="T4" />
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
