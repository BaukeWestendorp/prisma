use yew::prelude::*;

use crate::components::layer_list::LayerList;
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
                    <div class="content">{
                        match ui_state.selected_layer {
                            Some(index) => format!("{index}"),
                            None => format!("None"),
                        }
                    }</div>
                </main>
            </ContextProvider<ProjectContext>>
        </ContextProvider<UiStateContext>>
    }
}
