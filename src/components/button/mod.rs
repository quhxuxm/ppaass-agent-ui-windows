use stylist::{StyleSource, yew::styled_component};
use yew::{Callback, Classes, html, Html, NodeRef, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub id: String,
    #[prop_or(Option::Some("Button".to_string()))]
    pub label: Option<String>,
    #[prop_or(Classes::new())]
    pub classes: Classes,
    #[prop_or_default]
    pub on_click: Callback<web_sys::MouseEvent, ()>,
    #[prop_or_default()]
    pub button_ref: NodeRef,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let style = StyleSource::try_from(include_str!("button.css")).unwrap();
    html! {
        <button ref={&props.button_ref} class={style} onclick={props.on_click.clone()}>
            {props.label.clone()}
        </button>
    }
}
