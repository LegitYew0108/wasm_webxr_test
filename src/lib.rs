use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue>{
    console::log_1(&"Starting WebXR support check".into());
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let xrsystem = window.navigator().xr();
    console::log_2(&"XR system: ".into(), &xrsystem);
    let promise_is_supported = xrsystem.is_session_supported(web_sys::XrSessionMode::ImmersiveVr);
    let is_supported = wasm_bindgen_futures::JsFuture::from(promise_is_supported).await?.as_bool();
    console::log_2(&"is_supported: ".into(), &is_supported.into());

    let default_val = document.create_element("h1")?;
    default_val.set_text_content(Some("Is XR supported?"));
    
    let val = document.create_element("p")?;
    match is_supported {
        Some(true) => val.set_class_name("supported"),
        Some(false) => val.set_class_name("not-supported"),
        None => val.set_class_name("unknown"),
    };

    body.append_child(&default_val)?;
    body.append_child(&val)?;

    Ok(())
}

