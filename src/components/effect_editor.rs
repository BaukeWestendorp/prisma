use common::color::Color;
use common::effect::Effect;
use yew::prelude::*;

use crate::editor::editor_project::LayerId;
use crate::state::project::{ProjectAction, ProjectContext};

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub layer_id: LayerId,
}

#[function_component]
pub fn EffectEditor(props: &Props) -> Html {
    let project_ctx = use_context::<ProjectContext>().expect("project context not found");

    let update = {
        let props = props.clone();
        Callback::from(move |_| {
            if let Some(layer) = project_ctx.editor_project.get_layer(props.layer_id) {
                let mut new_layer = layer.clone();
                match new_layer.effect_layer.effect {
                    Effect::StaticColor { color } => {}
                    Effect::Wave {
                        color,
                        wave_type,
                        repeats,
                    } => {
                        new_layer.effect_layer.effect = Effect::StaticColor {
                            color: Color::blue(),
                        }
                    }
                    Effect::Strobe { color, pulse_width } => {}
                }
                project_ctx.dispatch(ProjectAction::UpdateLayer(props.layer_id, new_layer))
            }
        })
    };

    html! {
        <div>
            <button onclick={update}>{"UPDATE"}</button>
        </div>
    }
}
