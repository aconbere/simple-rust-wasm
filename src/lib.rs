use wasm_bindgen::prelude::*;
use web_sys::console;

mod gui;
mod webgl;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console::log_1(&JsValue::from_str("Sample::start"));

    //let window = web_sys::window().unwrap();
    //let document = window.document().unwrap();
    //let body = document.body().unwrap();

    //let val = document.create_element("p")?;
    //val.set_inner_html("Hello from Rust!");
    //body.append_child(&val)?;

    // webgl::start_webgl()?;
    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "my",
        web_options,
        Box::new(|cc| Box::new(gui::MyApp::new(cc))),
    )
    .await?;

    Ok(())
}
