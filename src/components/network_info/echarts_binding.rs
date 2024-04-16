use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

#[wasm_bindgen]
extern "C" {
    /// This is the echarts global object "echarts", it have following methods:
    /// * **init**: Used to initialize the echarts instance.
    pub type EChartsGlobal;

    #[wasm_bindgen(js_name = echarts)]
    pub static ECHARTS_GLOBAL: EChartsGlobal;

    #[wasm_bindgen(method, js_name = "init")]
    pub fn init(
        this: &EChartsGlobal,
        target: HtmlDivElement,
        theme: String,
        option: JsValue,
    ) -> ECharts;

}

#[wasm_bindgen]
extern "C" {
    /// This is the type of echarts instance, it have following methods:
    /// * **setOption**: Used to set the options of the charts.
    pub type ECharts;
    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn setOption(this: &ECharts, option: JsValue, not_merge: bool, lazy_update: bool);

    #[wasm_bindgen(method, js_name = "dispose")]
    pub fn dispose(this: &ECharts);

}
