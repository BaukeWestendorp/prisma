use yew::prelude::*;

pub type UiStateContext = UseReducerHandle<UiState>;

pub enum UiAction {
    SelectLayer(Option<usize>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UiState {
    pub selected_layer: Option<usize>,
}

impl Reducible for UiState {
    type Action = UiAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let selected_layer = match action {
            UiAction::SelectLayer(index) => index,
        };

        Self { selected_layer }.into()
    }
}
