use macroquad::prelude::*;
use std::f32::consts::PI;

struct Ship {
    position: Vec2,
    velocity: Vec2,
    angle: f32,
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

fn draw_ship(position: Vec2, angle: f32) {
    let nose = Vec2::new(
        position.x + angle.cos() * 20.0,
        position.y + angle.sin() * 20.0,
    );
    // 2.618 = 5.0 * PI / 6.0
    let left_wing = Vec2::new(
        position.x + (angle - 2.618).cos() * 15.0,
        position.y + (angle - 2.618).sin() * 15.0,
    );
    let right_wing = Vec2::new(
        position.x + (angle + 5.0 * PI / 6.0).cos() * 15.0,
        position.y + (angle + 5.0 * PI / 6.0).sin() * 15.0,
    );
    draw_triangle(nose, left_wing, right_wing, WHITE);
}

// Tunable constants
const THRUST_POWER: f32 = 200.0; // Pixels per second²
// Acceleration (thrust) is in pixels per second²
// (how fast velocity changes)
const DRAG: f32 = 0.99;

#[macroquad::main("Asteroids")]
async fn main() {
    let mut ship = Ship {
        position: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        velocity: Vec2::new(0.0, 0.0),
        angle: 0.0,
    };

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        // Rotation
        let rotation_speed = 4.0;
        if is_key_down(KeyCode::Left) {
            ship.angle -= rotation_speed * dt
        }
        if is_key_down(KeyCode::Right) {
            ship.angle += rotation_speed * dt
        }

        // Thrust
        if is_key_down(KeyCode::Up) {
            ship.velocity.x += ship.angle.cos() * THRUST_POWER * dt;
            ship.velocity.y += ship.angle.sin() * THRUST_POWER * dt;
        }

        // Update position
        ship.position += ship.velocity * dt;
        wrap_position(&mut ship.position, screen_width(), screen_height());

        // Slight drag
        ship.velocity *= DRAG;

        // Draw the ship
        draw_ship(ship.position, ship.angle);

        next_frame().await;
    }
}
