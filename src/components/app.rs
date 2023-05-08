use yew::prelude::*;

use crate::components::layer_list::LayerList;

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <LayerList />
            <div class="content">{"Content"}</div>
        </main>
    }
}
