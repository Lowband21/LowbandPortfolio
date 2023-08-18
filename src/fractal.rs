use actix_web::{web, App, HttpServer, Responder};
use rayon::prelude::*;
use serde::Deserialize;
use std::time::Instant;

#[derive(Deserialize)]
pub struct FractalQuery {
    x: f64,
    y: f64,
    zoom: f64,
    width: Option<usize>,
    height: Option<usize>,
}

use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

const MAX_ITERATIONS: u32 = 1000;
const BLACK: &str = "rgb(0,0,0)";

#[derive(Serialize)]
struct Pixel {
    x: usize,
    y: usize,
    color: String,
}

pub async fn compute_fractal(web::Query(query): web::Query<FractalQuery>) -> impl Responder {
    let start = Instant::now();

    let width = query.width.unwrap_or(800);
    let height = query.height.unwrap_or(600);
    let mut pixels = Vec::new();

    let data: Vec<_> = (0..height)
        .into_par_iter()
        .flat_map(|pixel_y| {
            (0..width).into_par_iter().filter_map(move |pixel_x| {
                let x0 = (pixel_x as f64 / width as f64 - 0.5) * 2.0 / query.zoom + query.x;
                let y0 = (pixel_y as f64 / height as f64 - 0.5) * 2.0 / query.zoom + query.y;

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

                Some(Pixel {
                    x: pixel_x,
                    y: pixel_y,
                    color,
                })
            })
        })
        .collect();

    pixels.extend(data);

    let duration = start.elapsed();
    println!("Took: {}ms", duration.as_millis());

    HttpResponse::Ok().json(json!({
        "pixels": pixels,
        "width": width,
        "height": height
    }))
}
