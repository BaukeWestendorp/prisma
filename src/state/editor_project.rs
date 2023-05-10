use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};

use common::color::Color;
use common::effect::{Effect, EffectLayer, LedRange, WaveType};
use common::project::PrismaProject;

use yewdux::prelude::*;

use crate::invokations;

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct EditorProject {
    pub framerate: usize,
    pub global_bpm: f32,
    pub layers: Vec<Layer>, // TODO: Convert this to a hashmap for better performance.
}

impl EditorProject {
    pub fn total_layers(&self) -> usize {
        self.layers.len()
    }

    pub fn get_mut_layer_from_id(&mut self, id: LayerId) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|layer| layer.id == id)
    }

    pub fn get_layer_from_id(&self, id: LayerId) -> Option<&Layer> {
        self.layers.iter().find(|layer| layer.id == id)
    }
}

impl Default for EditorProject {
    fn default() -> Self {
        Self {
            framerate: 50,
            global_bpm: 60.0,
            layers: vec![],
        }
    }
}

impl Store for EditorProject {
    fn new() -> Self {
        init_listener(ProjectTransferer);
        Self::default()
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

#[allow(dead_code)]
pub enum ProjectAction {
    AddLayer(Layer),
    AddDefaultLayer(),
    RemoveLayer(LayerId),
    UpdateLayer(LayerId, Layer),
    SetLayerVisibility(LayerId, bool),
    ToggleLayerVisibility(LayerId),
}

impl Reducer<EditorProject> for ProjectAction {
    fn apply(self, mut editor_project: Rc<EditorProject>) -> Rc<EditorProject> {
        let state = Rc::make_mut(&mut editor_project);

        match self {
            ProjectAction::AddLayer(layer) => {
                state.layers.push(layer);
            }
            ProjectAction::AddDefaultLayer() => {
                let layer_name = format!("New Layer {}", state.total_layers());
                let layer = Layer::new(layer_name.as_str(), true);
                state.layers.push(layer);
            }
            ProjectAction::RemoveLayer(id) => state.layers.retain_mut(|layer| layer.id != id),
            ProjectAction::UpdateLayer(id, new_layer) => {
                if let Some(layer) = state.get_mut_layer_from_id(id) {
                    *layer = new_layer;
                }
            }
            ProjectAction::SetLayerVisibility(id, visible) => {
                if let Some(layer) = state.get_mut_layer_from_id(id) {
                    layer.visible = visible
                };
            }
            ProjectAction::ToggleLayerVisibility(id) => {
                if let Some(layer) = state.get_mut_layer_from_id(id) {
                    layer.visible = !layer.visible
                };
            }
        };

        editor_project
    }
}

pub type LayerId = usize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    pub id: LayerId,
    pub name: String,
    pub visible: bool,
    pub effect_layer: EffectLayer,
}

impl Default for Layer {
    fn default() -> Self {
        Layer::new("New Layer", true)
    }
}

impl Layer {
    pub fn new(name: &str, visible: bool) -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            name: String::from(name),
            visible,
            effect_layer: EffectLayer::new(
                1.0,
                LedRange { min: 0, max: 40 },
                Effect::Wave {
                    color: Color::red(),
                    wave_type: WaveType::Sine,
                    repeats: 1.0,
                },
            ),
        }
    }
}

struct ProjectTransferer;
impl Listener for ProjectTransferer {
    type Store = EditorProject;
    fn on_change(&mut self, state: Rc<Self::Store>) {
        invokations::update_project(state.as_ref());
    }
}

impl From<EditorProject> for PrismaProject {
    fn from(editor_project: EditorProject) -> Self {
        Self {
            framerate: editor_project.framerate,
            global_bpm: editor_project.global_bpm,
            effect_layers: editor_project
                .layers
                .into_iter()
                .filter_map(|layer| match layer.visible {
                    true => Some(layer.effect_layer),
                    false => None,
                })
                .collect::<Vec<_>>(),
        }
    }
}
