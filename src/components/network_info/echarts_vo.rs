#![allow(unused)]
use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct EchartGlobalInitOption {
    #[serde(rename = "devicePixelRatio")]
    pub device_pixel_ratio: Option<f64>,
    pub renderer: Option<String>,
    #[serde(rename = "useDirtyRect")]
    pub use_dirty_rect: Option<bool>,
    #[serde(rename = "useCoarsePointer")]
    pub use_coarse_pointer: Option<bool>,
    #[serde(rename = "pointerSize")]
    pub pointer_size: Option<u32>,
    pub ssr: Option<bool>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub locale: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EchartOption {
    #[serde(rename = "xAxis")]
    pub x_axis: EchartOptionXAxis,
    #[serde(rename = "yAxis")]
    pub y_axis: EchartOptionYAxis,
    pub series: Vec<EchartOptionSeriesElement>,
    pub tooltip: Option<EchartOptionTooltip>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EchartOptionTooltip {
    pub show: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EchartOptionXAxis {
    pub data: Option<Vec<String>>,
    pub show: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EchartOptionAxisLabel {
    pub show: bool,
    pub formatter: String,
    #[serde(rename = "fontSize")]
    pub font_size: u32,
    #[serde(rename = "showMinLabel")]
    pub show_min_label: bool,
    #[serde(rename = "showMaxLabel")]
    pub show_max_label: bool,
    #[serde(rename = "alignMaxLabel")]
    pub align_max_label: String,
    pub margin: Option<u32>,
    pub align: Option<String>,
    pub inside: bool,
    pub overflow: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EchartOptionYAxis {
    pub data: Option<Vec<String>>,
    pub show: bool,
    pub interval: Option<f64>,
    #[serde(rename = "axisLabel")]
    pub axis_label: EchartOptionAxisLabel,
    pub offset: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct EchartOptionSeriesElement {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub chart_type: String,
    pub data: VecDeque<f64>,
    #[serde(rename = "areaStyle")]
    pub area_style: Option<EchartOptionSeriesElementAreaStyle>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EchartOptionSeriesElementAreaStyle {
    pub color: Option<String>,
    pub origin: Option<String>,
    #[serde(rename = "shadowBlur")]
    pub shadow_blur: Option<u32>,
    #[serde(rename = "shadowColor")]
    pub shadow_color: Option<String>,
    #[serde(rename = "shadowOffsetX")]
    pub shadow_offset_x: Option<u32>,
    #[serde(rename = "shadowOffsetY")]
    pub shadow_offset_y: Option<u32>,
    pub opacity: Option<f64>,
}
