use yew::prelude::*;

use crate::components::layer_list::LayerList;
use crate::components::main_content::MainContent;
use crate::state::project::{ProjectContext, ProjectState};
use crate::state::ui::{UiState, UiStateContext};

#[function_component]
pub fn App() -> Html {
    let project = use_reducer(|| ProjectState::default());
    let ui_state = use_reducer(|| UiState::default());
    html! {
        <ContextProvider<UiStateContext> context={ui_state.clone()}>
            <ContextProvider<ProjectContext> context={project}>
                <main>
                    <LayerList />
                    <MainContent />
                </main>
            </ContextProvider<ProjectContext>>
        </ContextProvider<UiStateContext>>
    }
}
