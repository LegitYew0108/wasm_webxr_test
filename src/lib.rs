use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue>{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let xrsystem = window.navigator().xr();
    let is_supported = xrsystem.is_session_supported(web_sys::XrSessionMode::ImmersiveVr).as_string();

    let val = document.create_element("p")?;
    match is_supported{
        Some(supported) => {
            val.set_inner_html(&format!("XR is supported: {}", supported));
        },
        None => {
            val.set_inner_html("None");
        }
    }

    body.append_child(&val)?;

    Ok(())
}
