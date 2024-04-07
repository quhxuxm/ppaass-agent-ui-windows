use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Register a callback listen to the tauri event
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name ="listen")]
    pub async fn listen_tauri_event(
        event_type: &str,
        callback: &Closure<dyn FnMut(JsValue)>,
    ) -> JsValue;

    /// Binding to the invoke method to call tauri backend with arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke", catch)]
    pub async fn invoke_tauri_with_arg(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    /// Binding to the invoke method to call tauri backend without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke", catch)]
    pub async fn invoke_tauri_without_arg(cmd: &str) -> Result<JsValue, JsValue>;

}
