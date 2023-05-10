use std::rc::Rc;

use yewdux::prelude::*;

use super::editor_project::LayerId;

#[derive(Clone, Debug, Default, PartialEq, Store)]
pub struct UiState {
    pub selected_layer: Option<LayerId>,
}

pub enum UiAction {
    SelectLayer(Option<LayerId>),
}

impl Reducer<UiState> for UiAction {
    fn apply(self, mut ui_state: Rc<UiState>) -> Rc<UiState> {
        let state = Rc::make_mut(&mut ui_state);
        match self {
            UiAction::SelectLayer(id) => state.selected_layer = id,
        }

        ui_state
    }
}
