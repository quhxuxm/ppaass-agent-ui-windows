use std::cell::RefCell;

use stylist::{yew::styled_component, StyleSource};
use web_sys::HtmlInputElement;
use yew::{classes, html, Callback, Classes, Html, NodeRef, Properties};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InputFieldDataType {
    Text,
    Number { min: i128, max: i128 },
}

#[derive(Properties, PartialEq)]
pub struct InputFieldProps {
    pub id: String,
    #[prop_or(Option::None)]
    pub label: Option<String>,
    #[prop_or(Option::None)]
    pub place_holder: Option<String>,
    #[prop_or(Option::None)]
    pub hint: Option<String>,
    #[prop_or(Option::None)]
    pub value: Option<String>,
    #[prop_or(Classes::new())]
    pub classes: Classes,
    #[prop_or_default()]
    pub input_ref: NodeRef,
    #[prop_or(InputFieldDataType::Text)]
    pub data_type: InputFieldDataType,
}

#[styled_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let input_state = match &props.value {
        Some(value) => RefCell::new(value.clone()),
        None => RefCell::new("".to_string()),
    };
    let on_change_callback = {
        let input_ref = props.input_ref.clone();
        let data_type = props.data_type;
        Callback::from(move |_| {
            let html_input = match input_ref.cast::<HtmlInputElement>() {
                Some(html_input) => html_input,
                None => return,
            };
            let input_value = html_input.value();
            if let InputFieldDataType::Number { min, max } = data_type {
                if input_value.is_empty() {
                    return;
                }
                match input_value.parse::<i128>() {
                    Ok(new_value) => {
                        {
                            let old_input_state = input_state.borrow();
                            gloo::console::info!(
                                "The old value: {}, the new value: {}",
                                &old_input_state.to_string(),
                                new_value
                            );
                            if min > new_value || max < new_value {
                                html_input.set_value(&old_input_state);
                                return;
                            }
                            html_input.set_value(new_value.to_string().as_str());
                        }
                        input_state.replace(new_value.to_string());
                    }
                    Err(e) => {
                        let old_input_state = input_state.borrow();
                        gloo::console::info!(
                            "Recover to old value: {} because of error: {}",
                            &old_input_state.to_string(),
                            e.to_string()
                        );
                        html_input.set_value(&old_input_state);
                    }
                }
            }
        })
    };
    let style = StyleSource::try_from(include_str!("input_field.css")).unwrap();
    html! {
        <div class={classes!(style, props.classes.clone())}>
            if let Some(label) = &props.label {
                <label for={props.id.clone()}>{label}</label>
            }
            <input id={props.id.clone()}
                type="text" ref={&props.input_ref}
                placeholder={props.place_holder.clone()}
                value={props.value.clone().unwrap_or("".to_string())}
                oninput={on_change_callback}
            />
            if let Some(hint) = &props.hint{
                <span class="hint">{hint}</span>
            }
        </div>

    }
}
