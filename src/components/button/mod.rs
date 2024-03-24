use stylist::{yew::styled_component, StyleSource};
use yew::{html, Callback, Classes, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub id: String,
    #[prop_or(Option::Some("Button".to_string()))]
    pub label: Option<String>,
    #[prop_or(Classes::new())]
    pub classes: Classes,
    #[prop_or_default]
    pub on_click: Callback<web_sys::MouseEvent, ()>,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let style = StyleSource::try_from(include_str!("button.css")).unwrap();
    html! {
        <button class={style} onclick={props.on_click.clone()}>
            {props.label.clone()}
        </button>
    }
}
