use std::rc::Rc;

use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    name: String,
    visible: bool,
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
        }
    }
}

enum LayersAction {
    Add(Layer),
    Remove(usize),
}

struct LayersState {
    layers: Vec<Layer>,
}

impl Default for LayersState {
    fn default() -> Self {
        Self {
            layers: vec![Layer::new("Base Layer", true)],
        }
    }
}

impl Reducible for LayersState {
    type Action = LayersAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new_layers = match action {
            LayersAction::Add(layer) => {
                let mut layers = self.layers.to_vec();
                layers.push(layer);
                layers
            }
            LayersAction::Remove(index) => {
                let mut layers = self.layers.to_vec();
                layers.remove(index);
                layers
            }
        };

        Self { layers: new_layers }.into()
    }
}

#[function_component]
pub fn LayerList() -> Html {
    let layers = use_reducer(LayersState::default);

    let add_layer = {
        let layers = layers.clone();

        Callback::from(move |_| {
            let layer = Layer::new(format!("New Layer {}", layers.layers.len()).as_str(), true);
            layers.dispatch(LayersAction::Add(layer))
        })
    };

    let remove_layer = |index| {
        let layers = layers.clone();
        Callback::from(move |_| layers.dispatch(LayersAction::Remove(index)))
    };

    html! {
        <div class="layer-list">
            {
                layers.layers.to_vec().into_iter().enumerate().map(|(index, layer)| {
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
