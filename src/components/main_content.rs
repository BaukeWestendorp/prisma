use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::layer_editor::LayerEditor;
use crate::state::editor_project::EditorProject;
use crate::state::ui::UiState;

#[function_component]
pub fn MainContent() -> Html {
    let project = use_store_value::<EditorProject>();
    let ui_state = use_store_value::<UiState>();

    match ui_state.selected_layer {
        Some(selected_layer) => match project.get_layer_from_id(selected_layer) {
            Some(layer) => html! {
                <LayerEditor layer_id={layer.id} />
            },
            None => html! {
                <p>{"Layer not found!"}</p>
            },
        },
        None => html! {
            <p>{"Select a layer to edit"}</p>
        },
    }
}
