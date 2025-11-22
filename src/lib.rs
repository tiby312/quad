mod utils;

use wasm_bindgen::prelude::*;

use gloo::events::*;
use gloo::console::*;

use std::f64;
use futures::*;

use wasm_bindgen::JsValue;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen(start)]
pub async fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();


    let (mut key_sender,key_receiver)=futures::channel::mpsc::channel(200);

    // Listen to "click" events on the button.
    let _a = EventListener::new(&document, "keydown", move |event| {
        let e:web_sys::KeyboardEvent=event.clone().dyn_into().unwrap();
        log!("key {:?}",e.key());
        if let Err(e)=key_sender.try_send(e){
           log!("failed to process {:?}",e.into_inner().key());
        }
    });

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();




    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();


    futures::join!(futures::future::pending::<()>());
}