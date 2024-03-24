use stylist::{yew::styled_component, StyleSource};
use yew::{classes, html, Classes, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or(Classes::new())]
    pub classes: Classes,
    pub children: Html,
}

#[styled_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let style = StyleSource::try_from(include_str!("container.css")).unwrap();
    html! {
        <div class={classes!(style, props.classes.clone())}>
            {props.children.clone()}
        </div>
    }
}
