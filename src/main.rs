mod components;
mod editor;
mod invokations;
mod state;

use components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
