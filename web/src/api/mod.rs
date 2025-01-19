use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn emit(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn listen(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct LoadArgs<'a> {
    locale: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct LoadResult {
    pub version: String
}

pub async fn load<'a>() -> Result<LoadResult, serde_wasm_bindgen::Error> {
    let args = LoadArgs { 
        locale: "" 
    };
    // let args = serde_wasm_bindgen::to_value(&args).unwrap();
    // let value = invoke("load", args).await;
    // serde_wasm_bindgen::from_value::<LoadResult>(value)
    Ok(LoadResult {
        version: "0.0.1".into()
    })
}