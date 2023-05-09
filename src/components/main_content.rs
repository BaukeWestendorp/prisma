use yew::prelude::*;

use crate::components::effect_editor::EffectEditor;
use crate::state::project::ProjectContext;
use crate::state::ui::UiStateContext;

#[function_component]
pub fn MainContent() -> Html {
    let project_ctx = use_context::<ProjectContext>().expect("no project context found");
    let ui_state_ctx = use_context::<UiStateContext>().expect("no ui state context found");

    match ui_state_ctx.selected_layer {
        Some(selected_layer) => match project_ctx.editor_project.layers.get(selected_layer) {
            Some(layer) => html! {
                <EffectEditor layer_id={layer.id} />
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
