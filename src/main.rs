mod app;
mod bo;
mod callbacks;
pub mod components;
mod wasm_binding;
use app::AgentServerConfigurationUi;

fn main() {
    yew::Renderer::<AgentServerConfigurationUi>::new().render();
}
