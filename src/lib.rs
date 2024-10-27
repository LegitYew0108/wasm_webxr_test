use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue>{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let xrsystem = window.navigator().xr();
    let promise_is_supported = xrsystem.is_session_supported(web_sys::XrSessionMode::ImmersiveVr);
    let is_supported = wasm_bindgen_futures::JsFuture::from(promise_is_supported).await?.as_bool();

    let val = document.create_element("p")?;
    match is_supported {
        Some(true) => val.set_class_name("supported"),
        Some(false) => val.set_class_name("not-supported"),
        None => val.set_class_name("unknown"),
    };

    body.append_child(&val)?;

    Ok(())
}

