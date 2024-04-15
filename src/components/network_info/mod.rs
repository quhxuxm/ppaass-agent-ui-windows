mod echarts_binding;
mod echarts_vo;

use std::collections::VecDeque;

use gloo::utils::format::JsValueSerdeExt;

use stylist::{yew::styled_component, StyleSource};
use wasm_bindgen::JsValue;
use web_sys::HtmlDivElement;
use yew::{classes, html, use_effect, use_node_ref, Html, Properties};

use crate::components::network_info::{
    echarts_binding::ECHARTS_GLOBAL,
    echarts_vo::{
        EchartGlobalInitOption, EchartOption, EchartOptionAxisLabel, EchartOptionSeriesElement,
        EchartOptionXAxis, EchartOptionYAxis,
    },
};

#[derive(Properties, PartialEq)]
pub struct NetworkInfoProps {
    pub download_content_data: VecDeque<f64>,
    pub upload_content_data: VecDeque<f64>,
}

#[styled_component(NetworkInfo)]
pub fn network_info(props: &NetworkInfoProps) -> Html {
    let chart_target = use_node_ref();
    let style = StyleSource::try_from(include_str!("network_info.css")).unwrap();
    {
        let chart_target = chart_target.clone();
        let download_content_data = props.download_content_data.clone();
        let upload_content_data = props.upload_content_data.clone();
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
                tooltip: None,
                x_axis: EchartOptionXAxis {
                    data: Some(vec![]),
                    show: true,
                },
                y_axis: EchartOptionYAxis {
                    data: None,
                    show: true,
                    interval: Some(f64::MAX),
                    offset: Some(-250),
                    axis_label: EchartOptionAxisLabel {
                        show: true,
                        formatter: "{value} MB/S".to_string(),
                        font_size: 10,
                        show_min_label: false,
                        show_max_label: true,
                        align_max_label: "right".to_string(),
                        margin: Some(3),
                        align: Some("right".to_string()),
                        inside: true,
                        overflow: Some("break".to_string()),
                    },
                },
                series: vec![
                    EchartOptionSeriesElement {
                        name: Some("Download speed".to_string()),
                        chart_type: "line".to_string(),
                        data: download_content_data,
                        area_style: Some(Default::default()),
                    },
                    EchartOptionSeriesElement {
                        name: Some("Upload speed".to_string()),
                        chart_type: "line".to_string(),
                        data: upload_content_data,
                        area_style: Some(Default::default()),
                    },
                ],
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
