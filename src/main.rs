use macroquad::prelude::*;

mod ship;
use ship::Ship;

#[macroquad::main("Asteroids")]
async fn main() {
    let mut ship = Ship::new();

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        // Gather input
        let input = ship::get_input();

        // Update
        ship.update(&input, dt);

        // Draw
        ship.draw();

        next_frame().await;
    }
}
