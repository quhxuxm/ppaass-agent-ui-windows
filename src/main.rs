mod app;
mod bo;
mod callbacks;
pub mod components;
mod wasm_binding;

use app::PpaassAgentUi;

fn main() {
    yew::Renderer::<PpaassAgentUi>::new().render();
}
