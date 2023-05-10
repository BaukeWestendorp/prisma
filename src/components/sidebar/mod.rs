use yew::prelude::*;

use layer_list::LayerList;

mod layer_list;

#[function_component]
pub fn Sidebar() -> Html {
    html! {
        <div class="sidebar">
            <h2>{"Layers"}</h2>
            <LayerList />
        </div>
    }
}
