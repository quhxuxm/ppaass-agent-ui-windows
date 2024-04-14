mod echarts_binding;
mod echarts_vo;

use gloo::utils::format::JsValueSerdeExt;
use js_sys::{Array, JsString, Map, Number, Object};
use stylist::{yew::styled_component, StyleSource};
use wasm_bindgen::JsValue;
use web_sys::HtmlDivElement;
use yew::{classes, html, use_effect, use_node_ref, Html, Properties};

use crate::components::network_info::{
    echarts_binding::ECHARTS_GLOBAL,
    echarts_vo::{
        EchartGlobalInitOption, EchartOption, EchartOptionSeriesElement, EchartOptionXAxis,
        EchartOptionYAxis,
    },
};

#[derive(Properties, PartialEq)]
pub struct NetworkInfoProps {}

#[styled_component(NetworkInfo)]
pub fn network_info(props: &NetworkInfoProps) -> Html {
    let chart_target = use_node_ref();
    let style = StyleSource::try_from(include_str!("network_info.css")).unwrap();
    {
        let chart_target = chart_target.clone();
        use_effect(move || {
            let chart_div = chart_target.cast::<HtmlDivElement>().unwrap();
            let global_init_option = EchartGlobalInitOption {
                width: Some("auto".to_string()),
                height: Some("250px".to_string()),
                ..Default::default()
            };
            let global_init_option = JsValue::from_serde(&global_init_option).unwrap();
            let echarts_instance =
                ECHARTS_GLOBAL.init(chart_div, "light".to_string(), global_init_option);
            let option = EchartOption {
                x_axis: EchartOptionXAxis {
                    data: Some(vec![]),
                    show: true,
                },
                y_axis: EchartOptionYAxis {
                    data: None,
                    show: true,
                    interval: f64::MAX,
                },
                series: vec![EchartOptionSeriesElement {
                    name: None,
                    chart_type: "line".to_string(),
                    data: vec![10, 20, 30, 25, 15, 5, 20],
                }],
            };
            let option = JsValue::from_serde(&option).unwrap();
            echarts_instance.setOption(option, true, false);
            || {}
        });
    }
    html! {
        <div class={classes!(style)} ref={chart_target}>
        </div>
    }
}
