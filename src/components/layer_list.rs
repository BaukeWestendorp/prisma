use yew::prelude::*;

use crate::editor::editor_project::Layer;
use crate::invokations;
use crate::state::project::{ProjectAction, ProjectContext};
use crate::state::ui::{UiAction, UiStateContext};

#[function_component]
pub fn LayerList() -> Html {
    let project_ctx = use_context::<ProjectContext>().expect("no project context found");
    let ui_state_ctx = use_context::<UiStateContext>().expect("no ui state context found");

    let add_layer = {
        let project_ctx = project_ctx.clone();
        Callback::from(move |_| {
            let layer = Layer::new(
                format!("New Layer {}", project_ctx.editor_project.layers.len()).as_str(),
                true,
            );
            project_ctx.dispatch(ProjectAction::Add(layer))
        })
    };

    let remove_layer = |index| {
        let project_ctx = project_ctx.clone();
        Callback::from(move |_| project_ctx.dispatch(ProjectAction::Remove(index)))
    };

    let toggle_layer_visibility = |index| {
        let project_ctx = project_ctx.clone();
        let visible = match project_ctx.editor_project.layers.get(index) {
            Some(layer) => !(layer as &Layer).visible,
            None => false,
        };
        Callback::from(move |_| {
            project_ctx.dispatch(ProjectAction::SetLayerVisibility(index, visible))
        })
    };

    let select_layer = |index: usize| {
        let ui_state_ctx = ui_state_ctx.clone();
        Callback::from(move |_| ui_state_ctx.dispatch(UiAction::SelectLayer(Some(index))))
    };

    use_effect_with_deps(
        {
            let project_ctx = project_ctx.clone();
            move |_| {
                invokations::update_project(project_ctx.editor_project.clone());
            }
        },
        project_ctx.clone(),
    );

    use_effect_with_deps(
        {
            let project_ctx = project_ctx.clone();
            let ui_state_ctx = ui_state_ctx.clone();
            move |_| {
                if let Some(selected_layer) = ui_state_ctx.selected_layer {
                    if project_ctx
                        .editor_project
                        .layers
                        .get(selected_layer)
                        .is_none()
                    {
                        ui_state_ctx.dispatch(UiAction::SelectLayer(None))
                    }
                }
            }
        },
        project_ctx.editor_project.layers.clone(),
    );

    html! {
        <div class="layer-list">
            {
                project_ctx.editor_project.layers.iter().enumerate().map(|(index, layer)| {
                    html! {
                        <div class="layer" onclick={select_layer(index)}>
                            <h3>{layer.clone().name}</h3>
                            <label>
                                {"Visible"}
                                <input
                                    type="checkbox"
                                    checked={layer.visible}
                                    onchange={toggle_layer_visibility(index)}
                                />
                            </label>
                            <button onclick={remove_layer(index)}>{"Remove"}</button>
                        </div>
                    }
                }).collect::<Html>()
            }

            <button onclick={add_layer}>{"Add Layer"}</button>
        </div>
    }
}
