use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

#[wasm_bindgen]
extern "C" {
    pub type ECharts;

    #[wasm_bindgen(js_namespace = ["echarts"], js_name ="init")]
    pub fn init(target: HtmlDivElement) -> ECharts;
}
