use common::color::Color;
use common::effect::{Effect, EffectLayer, LedRange, WaveType};
use common::project::Project;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct EditorProject {
    pub framerate: usize,
    pub global_bpm: f32,
    pub layers: Vec<Layer>,
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
                .map(|layer| layer.effect_layer)
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Layer {
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
            name: String::from(name),
            visible,
            effect_layer: EffectLayer {
                bpm_factor: 1.0,
                range: LedRange { min: 0, max: 40 },
                effect: Effect::Wave {
                    color: Color::red(),
                    wave_type: WaveType::Sine,
                    repeats: 1.0,
                },
            },
        }
    }
}
