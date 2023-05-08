use yew::prelude::*;

use crate::editor::editor_project::Layer;
use crate::invokations;
use crate::project_state::{ProjectAction, ProjectState};

#[function_component]
pub fn LayerList() -> Html {
    let project = use_reducer(ProjectState::default);

    let add_layer = {
        let project = project.clone();
        Callback::from(move |_| {
            let layer = Layer::new(
                format!("New Layer {}", project.project.layers.len()).as_str(),
                true,
            );
            project.dispatch(ProjectAction::Add(layer))
        })
    };

    let remove_layer = |index| {
        let project = project.clone();
        Callback::from(move |_| project.dispatch(ProjectAction::Remove(index)))
    };

    let toggle_layer_visibility = |index| {
        let project = project.clone();
        let visible = match project.project.layers.get(index) {
            Some(layer) => !(layer as &Layer).visible,
            None => false,
        };
        Callback::from(move |_| project.dispatch(ProjectAction::SetLayerVisibility(index, visible)))
    };

    use_effect_with_deps(
        {
            let project = project.clone();
            move |_| {
                invokations::update_project(project.project.clone());
            }
        },
        project.clone(),
    );

    html! {
        <div class="layer-list">
            {
                project.project.layers.iter().enumerate().map(|(index, layer)| {
                    html! {
                        <div class="layer">
                            <h3>{layer.clone().name}</h3>
                            <label>
                                {"Visible"}
                                <input type="checkbox" checked={layer.visible} onchange={toggle_layer_visibility(index)} />
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
