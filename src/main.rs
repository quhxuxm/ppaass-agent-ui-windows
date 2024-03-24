mod app;
pub mod components;

use app::PpaassAgentUi;

fn main() {
    yew::Renderer::<PpaassAgentUi>::new().render();
}
