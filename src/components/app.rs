use yew::prelude::*;

use crate::components::layer_list::LayerList;
use crate::components::main_content::MainContent;

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <LayerList />
            <MainContent />
        </main>
    }
}
