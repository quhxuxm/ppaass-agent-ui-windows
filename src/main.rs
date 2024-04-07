mod app;
mod backend;
pub mod components;
mod wasm_binding;

use app::PpaassAgentUi;

fn main() {
    yew::Renderer::<PpaassAgentUi>::new().render();
}
