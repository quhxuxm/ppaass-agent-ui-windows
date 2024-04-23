mod echarts_binding;
mod echarts_vo;

use std::collections::VecDeque;

use gloo::utils::format::JsValueSerdeExt;

use stylist::StyleSource;
use wasm_bindgen::JsValue;
use web_sys::HtmlDivElement;
use yew::{
    classes, function_component, html, use_effect, use_node_ref, use_state, Html, Properties,
};

use crate::components::network_chart::{
    echarts_binding::{ECharts, ECHARTS_GLOBAL},
    echarts_vo::{
        EchartGlobalInitOption, EchartOption, EchartOptionAxisLabel, EchartOptionSeriesElement,
        EchartOptionXAxis, EchartOptionYAxis,
    },
};

#[derive(Properties, PartialEq)]
pub struct NetworkChartProps {
    // pub network_update_callback: Callback<NetworkUpdateInfo>,
    pub download_content_data: VecDeque<f64>,
    pub upload_content_data: VecDeque<f64>,
}

#[function_component(NetworkChart)]
pub fn network_chart(props: &NetworkChartProps) -> Html {
    let chart_target = use_node_ref();
    let style = StyleSource::try_from(include_str!("network_chart.css")).unwrap();

    let echarts_instance_option = EchartOption {
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
                data: props
                    .download_content_data
                    .clone()
                    .iter()
                    .map(|v| format!("{v:.2}"))
                    .collect::<Vec<String>>(),
                area_style: Some(Default::default()),
            },
            EchartOptionSeriesElement {
                name: Some("Upload speed".to_string()),
                chart_type: "line".to_string(),
                data: props
                    .upload_content_data
                    .clone()
                    .iter()
                    .map(|v| format!("{v:.2}"))
                    .collect::<Vec<String>>(),
                area_style: Some(Default::default()),
            },
        ],
    };
    let echarts_instance_state = use_state(|| Option::<ECharts>::None);
    let chart_target_clone = chart_target.clone();
    use_effect(move || {
        match *echarts_instance_state {
            Some(ref echarts_instance) => {
                let option = JsValue::from_serde(&echarts_instance_option).unwrap();
                echarts_instance.setOption(&option, true, false);
            }
            None => {
                let echarts_global_init_option = EchartGlobalInitOption {
                    width: Some("auto".to_string()),
                    height: Some("250px".to_string()),
                    ..Default::default()
                };
                let echarts_global_init_option =
                    JsValue::from_serde(&echarts_global_init_option).unwrap();
                let chart_div = chart_target_clone.cast::<HtmlDivElement>().unwrap();
                let echarts_instance =
                    ECHARTS_GLOBAL.init(chart_div, "light".to_string(), echarts_global_init_option);
                echarts_instance_state.set(Some(echarts_instance));
            }
        }

        || {}
    });

    html! {
        <div class={classes!(style)} ref={chart_target}>
        </div>
    }
}
