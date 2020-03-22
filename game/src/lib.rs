use std::f64;
use console_error_panic_hook;
use wasm_bindgen::prelude::*;

mod canvas;
use canvas::Canvas;

#[wasm_bindgen(start)]
pub fn start() {
    setup();
    run();
    
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn setup() {
    use web_sys::console;

    console_error_panic_hook::set_once();

    console::log_1(&"Running WebAssembly. Setting up game.".into());
}

fn run() {
    let canvas = Canvas::new("game-canvas", 400, 600, "black");
    canvas.ctx.begin_path();
    canvas.ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0).unwrap();
    canvas.ctx.stroke();
}
