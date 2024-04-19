use std::net::SocketAddr;

use stylist::{yew::styled_component, StyleSource};
use web_sys::HtmlInputElement;
use yew::{classes, html, use_state, Callback, Classes, Html, NodeRef, Properties};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InputFieldDataType {
    Text,
    Number { min: i128, max: i128 },
    Address,
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
    let is_error = use_state(|| false);
    let value_state = use_state(|| props.value.clone());

    let on_change_callback = {
        let is_error = is_error.clone();
        let value_state = value_state.clone();
        let input_ref = props.input_ref.clone();
        let data_type = props.data_type;
        Callback::from(move |_| {
            let html_input = match input_ref.cast::<HtmlInputElement>() {
                Some(html_input) => html_input,
                None => return,
            };
            let input_value = html_input.value();
            if input_value.is_empty() {
                gloo::console::info!("Input empty value");
                value_state.set(None);
                is_error.set(false);
                return;
            }
            match data_type {
                InputFieldDataType::Text => (),
                InputFieldDataType::Number { min, max } => match input_value.parse::<i128>() {
                    Ok(new_value) => {
                        gloo::console::info!("Input new value (i128):", new_value);
                        if min > new_value || max < new_value {
                            value_state.set(Some(new_value.to_string()));
                            is_error.set(true);
                            return;
                        }
                        value_state.set(Some(new_value.to_string()));
                        is_error.set(false);
                    }
                    Err(_) => {
                        gloo::console::info!(
                            "Input new value but have error on parse to i128:",
                            input_value.clone()
                        );
                        value_state.set(Some(input_value));
                        is_error.set(true);
                    }
                },
                InputFieldDataType::Address => {
                    match input_value.parse::<SocketAddr>() {
                        Ok(new_value) => {
                            gloo::console::info!(
                                "Input new value (address):",
                                new_value.to_string()
                            );
                            value_state.set(Some(new_value.to_string()));
                            is_error.set(false);
                        }
                        Err(_) => {
                            gloo::console::info!(
                                "Input new value but have error on parse to address:",
                                input_value.clone()
                            );
                            value_state.set(Some(input_value));
                            is_error.set(true);
                        }
                    };
                }
            }
        })
    };

    let style = StyleSource::try_from(include_str!("input_field.css")).unwrap();
    html! {
        <div class={classes!(style, props.classes.clone())}>
            if let Some(label) = &props.label {
                <label for={props.id.clone()} class={classes!(is_error.then_some("error"))}>{label}</label>
            }
            <input id={props.id.clone()}
                type="text" ref={&props.input_ref}
                class={classes!(is_error.then_some("error"))}
                placeholder={props.place_holder.clone()}
                value={(*value_state).clone()}
                oninput={on_change_callback}
            />
            if let Some(hint) = &props.hint{
                <span class={classes!("hint", is_error.then_some("error"))}>{hint}</span>
            }
        </div>

    }
}
