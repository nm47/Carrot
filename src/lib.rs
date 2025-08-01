use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use html2text;

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
    console_log!("Converting HTML to markdown...");
    html2text::from_read(html.as_bytes(), 80)
}

#[wasm_bindgen]
pub async fn parse_recipe(url: &str) -> Result<String, JsValue> {
    console_log!("Fetching URL: {}", url);
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    
    let request = Request::new_with_str_and_init(url, &opts)?;
    
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    
    let resp: Response = resp_value.dyn_into().unwrap();
    
    if !resp.ok() {
        return Err(JsValue::from_str(&format!("HTTP error: {}", resp.status())));
    }
    
    let text = JsFuture::from(resp.text()?).await?;
    let html = text.as_string().unwrap();
    
    console_log!("Converting HTML to markdown...");
    let markdown = html2text::from_read(html.as_bytes(), 80);
    
    Ok(markdown)
}
