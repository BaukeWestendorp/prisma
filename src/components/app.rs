use std::rc::Rc;

use yew::prelude::*;

use crate::editor::editor_project::{EditorProject, Layer};
use crate::invokations;

enum ProjectAction {
    Add(Layer),
    Remove(usize),
}

#[derive(Debug, Default, Clone, PartialEq)]
struct ProjectState {
    project: EditorProject,
}

impl Reducible for ProjectState {
    type Action = ProjectAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new_layers = match action {
            ProjectAction::Add(layer) => {
                let mut layers = self.project.layers.to_vec();
                layers.push(layer);
                layers
            }
            ProjectAction::Remove(index) => {
                let mut layers = self.project.layers.to_vec();
                layers.remove(index);
                layers
            }
        };

        Self {
            project: EditorProject {
                layers: new_layers,
                ..self.project
            },
        }
        .into()
    }
}

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
                project.project.layers.iter().cloned().enumerate().map(|(index, layer)| {
                    html! {
                        <div class="layer">
                            <h3>{layer.name}</h3>
                            <label>
                                {"Visible"}
                                <input type="checkbox" checked={layer.visible} />
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

#[function_component]
pub fn App() -> Html {
    html! {
        <main>
            <LayerList />
            <div class="content">{"Content"}</div>
        </main>
    }
}
