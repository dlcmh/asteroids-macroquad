use macroquad::prelude::*;

#[macroquad::main("Asteroids")]
async fn main() {
    loop {
        // 1. Process input
        // 2. Update state
        // 3. Draw everything

        clear_background(BLACK);
        draw_text("Hello, Asteroids!", 20.0, 20.0, 30.0, WHITE);

        // Center of the screen
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;

        // Draw a crosshair at center
        draw_line(cx - 20.0, cy, cx + 20.0, cy, 2.0, WHITE);
        draw_line(cx, cy - 20.0, cx, cy + 20.0, 2.0, WHITE);
        draw_circle(cx, cy, 5.0, GREEN);

        // Draw our "ship" for now - just a circle
        draw_circle(cx, cy - 50.0, 15.0, YELLOW);

        next_frame().await;
    }
}
