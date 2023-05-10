use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::state::editor_project::{EditorProject, ProjectAction};
use crate::state::ui::{UiAction, UiState};

#[function_component]
pub fn LayerList() -> Html {
    let (project, dispatch_project) = use_store::<EditorProject>();
    let (ui_state, dispatch_ui_state) = use_store::<UiState>();

    let add_layer = dispatch_project.apply_callback(move |_| ProjectAction::AddDefaultLayer());

    let remove_layer = |id| {
        dispatch_project.apply_callback(move |event: MouseEvent| {
            event.stop_propagation();
            ProjectAction::RemoveLayer(id)
        })
    };

    let toggle_layer_visibility =
        |id| dispatch_project.apply_callback(move |_| ProjectAction::ToggleLayerVisibility(id));

    let select_layer =
        |id| dispatch_ui_state.apply_callback(move |_| UiAction::SelectLayer(Some(id)));

    use_effect_with_deps(
        {
            let dispatch_ui_state = dispatch_ui_state.clone();
            let project = project.clone();
            let ui_state = ui_state.clone();
            move |_| {
                if let Some(selected_layer) = ui_state.selected_layer {
                    if project.get_layer_from_id(selected_layer).is_none() {
                        dispatch_ui_state.apply(UiAction::SelectLayer(None))
                    }
                }
            }
        },
        project.layers.clone(),
    );

    html! {
        <div class="layer-list">
            {
                project.layers.iter().map(move |layer| {
                    let is_selected = ui_state.selected_layer == Some(layer.id);
                    let selected_class = match is_selected {
                        true => Some("selected"),
                        false => None,
                    };

                    html! {
                        <div class={classes!("layer", selected_class)} onclick={select_layer(layer.id)}>
                            <h3>{layer.clone().name}</h3>
                            <label>
                                {"Visible"}
                                <input
                                    type="checkbox"
                                    checked={layer.visible}
                                    onchange={toggle_layer_visibility(layer.id)}
                                />
                            </label>
                            <button onclick={remove_layer(layer.id)}>{"Remove"}</button>
                            <span>{format!("(id: {})", layer.id)}</span>
                        </div>
                    }
                }).collect::<Html>()
            }

            <button onclick={add_layer}>{"Add Layer"}</button>
        </div>
    }
}
