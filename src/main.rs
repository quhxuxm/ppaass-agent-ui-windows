mod app;
mod bo;
pub mod components;
mod wasm_binding;

use app::PpaassAgentUi;

fn main() {
    yew::Renderer::<PpaassAgentUi>::new().render();
}
