use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue>{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let xrsystem = window.navigator().xr();
    let promise_is_supported = xrsystem.is_session_supported(web_sys::XrSessionMode::ImmersiveVr);
    let is_supported = wasm_bindgen_futures::JsFuture::from(promise_is_supported).await?.is_string();

    let val = document.create_element("p")?;
    val.set_inner_html(&format!("XR is supported: {}", is_supported));

    body.append_child(&val)?;

    Ok(())
}

