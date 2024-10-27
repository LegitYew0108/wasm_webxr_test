use wasm_bindgen::prelude::*;
use tracing::{span, Level};

#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue>{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let xrsystem = window.navigator().xr();
    tracing::debug!("xrsystem: {:?}", xrsystem);
    let promise_is_supported = xrsystem.is_session_supported(web_sys::XrSessionMode::ImmersiveVr);
    let is_supported = wasm_bindgen_futures::JsFuture::from(promise_is_supported).await?.is_string();

    let val = document.create_element("p")?;
    val.set_inner_html(&format!("XR is supported: {}", is_supported));

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen(start)]
pub async fn init() -> Result<(), JsValue> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let span = span!(Level::INFO, "init");
    let _enter = span.enter();

    run().await
}
