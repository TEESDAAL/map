use ::rand::prelude::*;
use macroquad::prelude::*;
use noise::{NoiseFn, PerlinSurflet};
 
#[macroquad::main("map")]
async fn main() {
    let mut noise = PerlinSurflet::new((0..1000).choose(&mut thread_rng()).unwrap());
    let (mut base_x, mut base_y) = (-(screen_width() / 2.).round() as i32, -(screen_height() / 2.).round() as i32);
    let mut zoom = 5.;
    let mut sea_level = 0.05;
    let mut strength_of_island_formation = 2.3 - f64::EPSILON;
    let mut island_radius = 4.;
    
    loop {
        clear_background(BLUE);
        let (width, height) = (screen_width() as f64, screen_height() as f64);
        key_presses(&mut noise, &mut base_x, &mut base_y, &mut zoom, &mut sea_level, &mut strength_of_island_formation, &mut island_radius);
        for x in 0..(width.round() as i32) {
            for y in 0..(height.round() as i32) {
                let (x_pos, y_pos) = (x + base_x, y + base_y);
                let elevation = get_elevation(&x_pos, &y_pos, &noise, &zoom, &strength_of_island_formation, &island_radius);
                if elevation > sea_level + 0.4 {
                    draw_rectangle(x as f32, y as f32, 1., 1., LIGHTGRAY);
                } else if elevation >= sea_level + 0.1 {
                    draw_rectangle(x as f32, y as f32, 1., 1., GREEN);
                } else if elevation >= sea_level {
                    draw_rectangle(x as f32, y as f32, 1., 1., BEIGE);
                }
            }
        }
        next_frame().await
    }
}

fn get_elevation(x: &i32, y: &i32, noise: &PerlinSurflet, zoom: &f64, strength_of_island_formation: &f64, island_radius: &f64 ) -> f64 {
    let (width, height) = (screen_width() as f64, screen_height() as f64);
    let xn = zoom * *x as f64 / width;
    let yn = zoom * *y as f64 / height;
    let distance_from_orgin = f64::min(1., (xn*xn + yn*yn) as f64 / f64::sqrt(*island_radius));
    let unmodified_elevation = noise.get([xn, yn]) + 0.5 * noise.get([2. * xn, 2. * yn]) + 0.25 * noise.get([4. * xn, 4. * yn]);
    (unmodified_elevation + 1. - distance_from_orgin * strength_of_island_formation) / 2.

}

fn key_presses(perlin: &mut PerlinSurflet, base_x: &mut i32, base_y: &mut i32, zoom: &mut f64, sea_level: &mut f64, strength_of_island_formation: &mut f64, island_radius: &mut f64) {
    if is_key_pressed(KeyCode::Space) {
        *perlin = PerlinSurflet::new((0..1000).choose(&mut thread_rng()).unwrap());
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
    if is_key_down(KeyCode::R) {
        *island_radius += 1.;
    }
    if is_key_down(KeyCode::E) {
        *island_radius -= 1.;
    }

    if is_key_down(KeyCode::I) {
        *strength_of_island_formation += 0.01;
        println!("{strength_of_island_formation}");
    }
    if is_key_down(KeyCode::O) {
        *strength_of_island_formation -= 0.01;
        println!("{strength_of_island_formation}");
    }
}
