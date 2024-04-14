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
}

#[derive(Serialize, Deserialize)]
pub struct EchartOptionXAxis {
    pub data: Option<Vec<String>>,
    pub show: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EchartOptionYAxis {
    pub data: Option<Vec<String>>,
    pub show: bool,
    pub interval: f64,
}

#[derive(Serialize, Deserialize)]
pub struct EchartOptionSeriesElement {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub chart_type: String,
    pub data: Vec<u32>,
}
