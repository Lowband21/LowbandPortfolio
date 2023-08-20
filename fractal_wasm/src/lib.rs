use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct Pixel {
    x: usize,
    y: usize,
    color: String,
}

#[wasm_bindgen]
impl Pixel {
    pub fn new(x: usize, y: usize, color: String) -> Self {
        Pixel { x, y, color }
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    pub fn color(&self) -> String {
        self.color.clone()
    }
}

#[wasm_bindgen]
pub fn compute_fractal(x: f64, y: f64, zoom: f64, width: usize, height: usize) -> JsValue {
    const MAX_ITERATIONS: usize = 1000;
    const BLACK: &str = "rgb(0,0,0)";

    let mut result = Vec::new();

    for pixel_y in 0..height {
        for pixel_x in 0..width {
            let x0 = (pixel_x as f64 / width as f64 - 0.5) * 2.0 * zoom + x;
            let y0 = (pixel_y as f64 / height as f64 - 0.5) * 2.0 * zoom + y;

            let mut x = 0.0;
            let mut y = 0.0;
            let mut iterations = 0;

            while x * x + y * y <= 4.0 && iterations < MAX_ITERATIONS {
                let temp_x = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = temp_x;
                iterations += 1;
            }

            let color = if iterations == MAX_ITERATIONS {
                BLACK.to_string()
            } else {
                let brightness = (iterations as f64 / MAX_ITERATIONS as f64 * 255.0) as u32;
                format!("rgb({},{},{})", brightness, brightness, brightness)
            };

            let pixel = Pixel::new(pixel_x, pixel_y, color);
            result.push(pixel);
        }
    }

    // Convert the data to a JS value using serde_wasm_bindgen.
    to_value(&result).unwrap()
}
