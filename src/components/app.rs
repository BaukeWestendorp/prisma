use yew::prelude::*;

use crate::components::main_content::MainContent;
use crate::components::sidebar::Sidebar;

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <Sidebar />
            <MainContent />
        </main>
    }
}
