use ::rand::prelude::*;
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::HashMap;
#[macroquad::main("map")]
async fn main() {
    let mut cache: HashMap<(i32, i32), f64> = HashMap::new();
    let mut perlin = Perlin::new((0..1000).choose(&mut thread_rng()).unwrap());
    let (mut base_x, mut base_y) = (0, 0);
    loop {
        clear_background(BLUE);
        if is_key_pressed(KeyCode::Space) {
            perlin = Perlin::new((0..1000).choose(&mut thread_rng()).unwrap());
            cache = HashMap::new();
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            base_y -= 10;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            base_y += 10;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            base_x -= 10;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            base_x += 10;
        }
        let (width, height) = (screen_width() as f64, screen_height() as f64);
        for x in 0..(width.round() as i32) {
            for y in 0..(height.round() as i32) {
                let (x_pos, y_pos) = (x + base_x, y + base_y);
                let val = match cache.get(&(x_pos, y_pos)) {
                    Some(val) => *val,
                    None => {
                        let xn = 2. * x_pos as f64 / width - 0.5;
                        let yn = 2. * y_pos as f64 / height - 0.5;
                        perlin.get([xn, yn])
                    }
                };
                if val > 0.7 {
                    draw_rectangle(x as f32, y as f32, 1., 1., LIGHTGRAY);
                } else if val > 0.2 {
                    draw_rectangle(x as f32, y as f32, 1., 1., GREEN);
                } else if val > 0.1 {
                    draw_rectangle(x as f32, y as f32, 1., 1., BEIGE);
                }
            }
        }
        next_frame().await
    }
}
