use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use crate::parse_recipe_from_content;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn parse_html(html: &str) -> String {
    console_log!("Parsing HTML structure...");
    
    // Use unified core function
    parse_recipe_from_content(html, "markdown")
}

#[wasm_bindgen]
pub fn parse_html_debug(_html: &str) -> String {
    console_log!("Debug mode not implemented yet");
    "Debug mode not implemented yet".to_string()
}

#[wasm_bindgen]
pub async fn parse_recipe(url: &str, format: &str) -> Result<String, JsValue> {
    console_log!("Fetching URL via proxy: {}", url);
    
    // Construct proxy URL
    let proxy_url = format!("/proxy?url={}", js_sys::encode_uri_component(url));
    console_log!("Proxy URL: {}", proxy_url);
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    
    let request = Request::new_with_str_and_init(&proxy_url, &opts)?;
    
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    
    let resp: Response = resp_value.dyn_into().unwrap();
    
    if !resp.ok() {
        return Err(JsValue::from_str(&format!("HTTP error: {}", resp.status())));
    }
    
    let text = JsFuture::from(resp.text()?).await?;
    let json_str = text.as_string().unwrap();
    
    // Parse JSON response from proxy
    let json_value: serde_json::Value = serde_json::from_str(&json_str)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;
    
    let html = json_value["contents"]
        .as_str()
        .ok_or_else(|| JsValue::from_str("No contents field in proxy response"))?;
    
    console_log!("Parsing HTML with format: {}", format);
    
    // Use unified core parsing function
    Ok(parse_recipe_from_content(html, format))
}