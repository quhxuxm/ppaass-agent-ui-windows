use stylist::{yew::styled_component, StyleSource};
use yew::{classes, html, Classes, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InputFieldProps {
    pub id: String,
    pub label: Option<String>,
    pub place_holder: Option<String>,
    pub hint: Option<String>,
    pub classes: Classes,
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
                <input id={props.id.clone()} type="text" placeholder={place_holder.clone()} />
            } else{
                <input id={props.id.clone()} type="text" />
            }
            if let Some(hint) = &props.hint{
                <span class="hint">{hint}</span>
            }
        </div>

    }
}
