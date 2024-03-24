use stylist::{yew::styled_component, StyleSource};
use yew::{html, Classes, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub id: String,
    pub label: Option<String>,
    pub classes: Classes,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let style = StyleSource::try_from(include_str!("button.css")).unwrap();
    html! {
        <button class={style}>{props.label.clone()}</button>
    }
}
