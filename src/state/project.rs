use common::effect::EffectLayer;
use yew::prelude::*;

use crate::editor::editor_project::{EditorProject, Layer, LayerId};

pub type ProjectContext = UseReducerHandle<ProjectState>;

pub enum ProjectAction {
    Add(Layer),
    Remove(usize),
    SetLayerVisibility(usize, bool),
    UpdateLayer(LayerId, Layer),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProjectState {
    pub editor_project: EditorProject,
}

impl Reducible for ProjectState {
    type Action = ProjectAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let new_layers = match action {
            ProjectAction::Add(layer) => {
                let mut layers = self.editor_project.layers.to_vec();
                layers.push(layer);
                layers
            }
            ProjectAction::Remove(index) => {
                let mut layers = self.editor_project.layers.to_vec();
                layers.remove(index);
                layers
            }
            ProjectAction::SetLayerVisibility(index, visible) => {
                let mut layers = self.editor_project.layers.to_vec();
                if let Some(layer) = layers.get_mut(index) {
                    layer.visible = visible
                };
                layers
            }
            ProjectAction::UpdateLayer(id, new_layer) => {
                let mut layers = self.editor_project.layers.to_vec();
                if let Some(layer) = layers.iter_mut().find(|layer| layer.id == id) {
                    *layer = new_layer;
                }
                layers
            }
        };

        Self {
            editor_project: EditorProject {
                layers: new_layers,
                ..self.editor_project
            },
        }
        .into()
    }
}
