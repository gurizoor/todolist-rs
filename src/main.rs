
mod modules;

// use crate::modules::app::TodoApp;

fn main() {
    yew::Renderer::<modules::app::App>::new().render();
}
