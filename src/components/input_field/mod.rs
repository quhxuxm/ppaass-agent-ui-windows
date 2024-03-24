use stylist::{yew::styled_component, StyleSource};
use yew::{classes, html, Classes, Html, NodeRef, Properties};

#[derive(Properties, PartialEq)]
pub struct InputFieldProps {
    pub id: String,
    #[prop_or(Option::None)]
    pub label: Option<String>,
    #[prop_or(Option::None)]
    pub place_holder: Option<String>,
    #[prop_or(Option::None)]
    pub hint: Option<String>,
    #[prop_or(Classes::new())]
    pub classes: Classes,
    #[prop_or_default()]
    pub input_ref: NodeRef,
}

#[styled_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let style = StyleSource::try_from(include_str!("input_field.css")).unwrap();
    html! {
        <div class={classes!(style, props.classes.clone())}>
            if let Some(label) = &props.label {
                <label for={props.id.clone()}>{label}</label>
            }
            if let Some(place_holder) = &props.place_holder{
                <input id={props.id.clone()} type="text" placeholder={place_holder.clone()} ref={&props.input_ref} />
            } else{
                <input id={props.id.clone()} type="text" ref={&props.input_ref} />
            }
            if let Some(hint) = &props.hint{
                <span class="hint">{hint}</span>
            }
        </div>

    }
}
