use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

pub struct Canvas {
    pub element: HtmlCanvasElement,
    pub ctx: CanvasRenderingContext2d,
    pub background_color: String,
    width: u32,
    height: u32,
    scaled_width: u32,
    scaled_height: u32,
}

impl Canvas {
    pub fn new(
        element_id: &str,
        width: u32,
        height: u32,
        background_color: &str
        ) -> Canvas {

        let document = window().unwrap().document().unwrap();
        let element = document.get_element_by_id(element_id).unwrap();

        let canvas: HtmlCanvasElement = element
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;

        Canvas {
            element: canvas,
            ctx,
            background_color: background_color.to_string(),
            width,
            height,
            scaled_width,
            scaled_height,
        }
    }

    pub fn run() {}

    fn clear(&self) {
        self.ctx.clear_rect(
            0.0,
            0.0,
            f64::from(self.width),
            f64::from(self.height)
        );
    }
}

