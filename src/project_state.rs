use std::rc::Rc;

use yew::prelude::*;

use crate::editor::editor_project::{EditorProject, Layer};

pub(crate) enum ProjectAction {
    Add(Layer),
    Remove(usize),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct ProjectState {
    pub(crate) project: EditorProject,
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
