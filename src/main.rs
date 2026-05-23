use macroquad::prelude::*;

#[macroquad::main("Asteroids")]
async fn main() {
    loop {
        // 1. Process input
        // 2. Update state
        // 3. Draw everything

        clear_background(BLACK);
        draw_text("Hello, Asteroids!", 20.0, 20.0, 30.0, WHITE);

        next_frame().await;
    }
}
