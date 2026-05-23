use macroquad::prelude::*;

struct Ship {
    position: Vec2,
    velocity: Vec2,
}

fn wrap_position(pos: &mut Vec2, screen_w: f32, screen_h: f32) {
    if pos.x < 0.0 {
        pos.x += screen_w
    }
    if pos.x > screen_w {
        pos.x -= screen_w
    }
    if pos.y < 0.0 {
        pos.y += screen_h
    }
    if pos.y > screen_h {
        pos.y -= screen_h
    }
}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut ship = Ship {
        position: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        velocity: Vec2::new(100.0, 50.0), // Moving right and down
    };

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        // Keyboard control
        let thrust_power = 200.0; // Pixels per second²
        // Acceleration (thrust) is in pixels per second²
        // (how fast velocity changes)
        if is_key_down(KeyCode::Right) {
            ship.velocity.x += thrust_power * dt
        }
        if is_key_down(KeyCode::Left) {
            ship.velocity.x -= thrust_power * dt
        }
        if is_key_down(KeyCode::Up) {
            ship.velocity.y -= thrust_power * dt // Negative is UP
        }
        if is_key_down(KeyCode::Down) {
            ship.velocity.y += thrust_power * dt // Positive is DOWN
        }

        // Update: Euler integration
        ship.position += ship.velocity * dt;

        // Apply slight drag each next_frame
        let drag = 0.99; // Multiply velocity by this each frame
        ship.velocity *= drag;

        // Update: Screen wrapping
        wrap_position(&mut ship.position, screen_width(), screen_height());

        // Draw the ship (a circle for now)
        draw_circle(ship.position.x, ship.position.y, 15.0, YELLOW);

        // Draw velocity vector as a line
        draw_line(
            ship.position.x,
            ship.position.y,
            ship.position.x + ship.velocity.x * 0.1,
            ship.position.y + ship.velocity.y * 0.1,
            2.0,
            RED,
        );

        next_frame().await;
    }
}
