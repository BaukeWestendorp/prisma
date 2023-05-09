use std::sync::atomic::{AtomicUsize, Ordering};

use common::color::Color;
use common::effect::{Effect, EffectLayer, LedRange, WaveType};
use common::project::Project;

pub type LayerId = usize;
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct EditorProject {
    pub framerate: usize,
    pub global_bpm: f32,
    pub layers: Vec<Layer>,
}

impl EditorProject {
    pub fn get_layer(&self, id: LayerId) -> Option<&Layer> {
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

impl From<EditorProject> for Project {
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Layer {
    pub id: usize,
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
