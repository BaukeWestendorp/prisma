mod components;
mod editor;
mod invokations;

use components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
