use common::color::Color;
use common::effect::Effect;

use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::state::project::{LayerId, ProjectAction, ProjectState};

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub layer_id: LayerId,
}

#[function_component]
pub fn LayerEditor(props: &Props) -> Html {
    let (project, dispatch_project) = use_store::<ProjectState>();

    let update = {
        let layer_id = props.layer_id;
        Callback::from(move |_| {
            if let Some(layer) = project.get_layer_from_id(layer_id) {
                let mut new_layer = layer.clone();
                new_layer.effect_layer.effect = Effect::Wave {
                    color: Color::green(),
                    wave_type: common::effect::WaveType::Square { pulse_width: 0.3 },
                    repeats: 1.0,
                };
                dispatch_project.apply(ProjectAction::UpdateLayer(layer_id, new_layer))
            }
        })
    };

    html! {
        <div>
            <button onclick={update}>{"UPDATE"}</button>
        </div>
    }
}
