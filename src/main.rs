use ::rand::prelude::*;
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
#[macroquad::main("map")]
async fn main() {
    let mut perlin = Perlin::new((0..1000).choose(&mut thread_rng()).unwrap());
    let (mut base_x, mut base_y) = (0, 0);
    let mut zoom = 5.;
    let mut sea_level = 0.1;
    loop {
        clear_background(BLUE);
        key_presses(&mut perlin, &mut base_x, &mut base_y, &mut zoom, &mut sea_level);
        let (width, height) = (screen_width() as f64, screen_height() as f64);
        for x in 0..(width.round() as i32) {
            for y in 0..(height.round() as i32) {
                let (x_pos, y_pos) = (x + base_x, y + base_y);
                let xn = zoom * x_pos as f64 / width;
                let yn = zoom * y_pos as f64 / height;
                let val = perlin.get([xn, yn]);
                if val > sea_level + 0.6 {
                    draw_rectangle(x as f32, y as f32, 1., 1., LIGHTGRAY);
                } else if val >= sea_level + 0.1 {
                    draw_rectangle(x as f32, y as f32, 1., 1., GREEN);
                } else if val >= sea_level {
                    draw_rectangle(x as f32, y as f32, 1., 1., BEIGE);
                }
            }
        }
        next_frame().await
    }
}

fn key_presses(perlin: &mut Perlin, base_x: &mut i32, base_y: &mut i32, zoom: &mut f64, sea_level: &mut f64) {
    if is_key_pressed(KeyCode::Space) {
        *perlin = Perlin::new((0..1000).choose(&mut thread_rng()).unwrap());
    }
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        *base_y -= 10;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        *base_y += 10;
    }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        *base_x -= 10;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        *base_x += 10;
    }
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Minus) {
        *zoom *= 2.;
    }
    if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::Equal) {
        *zoom /= 2.;
    }
    if is_key_down(KeyCode::J) {
        *sea_level -= 0.01;
    } else if is_key_down(KeyCode::K) {
        *sea_level += 0.01;
    }
}
