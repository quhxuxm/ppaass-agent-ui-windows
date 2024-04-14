use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct EchartGlobalInitOption {
    pub devicePixelRatio: Option<f64>,
    pub renderer: Option<String>,
    pub useDirtyRect: Option<bool>,
    pub useCoarsePointer: Option<bool>,
    pub pointerSize: Option<u32>,
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
