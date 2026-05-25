use std::f32::consts::PI;

use macroquad::prelude::*;

pub struct Ship {
    pub position: Vec2,
    pub velocity: Vec2,
    pub angle: f32,
    pub is_thrusting: bool,
    pub is_dead: bool,
    pub invincible_timer: f32,
}

pub struct ShipInput {
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub thrust: bool,
    pub shoot: bool,
}

const ROTATION_SPEED: f32 = 4.0;
const THRUST_POWER: f32 = 300.0; // Pixels per second²
// Acceleration (thrust) is in pixels per second²
// (how fast velocity changes)
const DRAG: f32 = 0.98;
const MAX_SPEED: f32 = 400.0;

impl Ship {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            velocity: Vec2::ZERO,
            angle: 0.0,
            is_thrusting: false,
            is_dead: false,
            invincible_timer: 0.0,
        }
    }

    pub fn update(&mut self, input: &ShipInput, dt: f32) {
        if self.is_dead {
            return;
        }

        // Count down invincibility
        if self.invincible_timer > 0.0 {
            self.invincible_timer -= dt
        }

        // Rotation
        if input.rotate_left {
            self.angle -= ROTATION_SPEED * dt
        }
        if input.rotate_right {
            self.angle += ROTATION_SPEED * dt
        }

        // Thrust
        self.is_thrusting = input.thrust;
        if self.is_thrusting {
            self.velocity.x += self.angle.cos() * THRUST_POWER * dt;
            self.velocity.y += self.angle.sin() * THRUST_POWER * dt;
        }

        // Drag and speed limit
        self.velocity *= DRAG;
        let speed = self.velocity.length();
        if speed > MAX_SPEED {
            self.velocity = self.velocity.normalize_or_zero() * MAX_SPEED;
        }

        // Move
        self.position += self.velocity * dt;
        wrap_position(&mut self.position);
    }

    pub fn draw(&self) {
        if self.is_dead {
            return;
        }

        // Blink when invincible
        if self.invincible_timer > 0.0 {
            let blink = (self.invincible_timer * 10.0) as i32 % 2 == 0;
            if blink {
                return; // Skip drawing this frame = blinking effect
            }
        }

        // Draw thrust flame first (behind the ship)
        if self.is_thrusting {
            draw_thrust_flame(self.position, self.angle);
        }

        // Draw the ship body
        draw_ship_body(self.position, self.angle);
    }
}

pub fn get_input() -> ShipInput {
    ShipInput {
        rotate_left: is_key_down(KeyCode::Left),
        rotate_right: is_key_down(KeyCode::Right),
        thrust: is_key_down(KeyCode::Up),
        shoot: is_key_pressed(KeyCode::Space),
    }
}

fn draw_ship_body(position: Vec2, angle: f32) {
    let nose = Vec2::new(
        position.x + angle.cos() * 20.0,
        position.y + angle.sin() * 20.0,
    );
    let left = Vec2::new(
        position.x + (angle - 2.618).cos() * 12.0,
        position.y + (angle - 2.618).sin() * 12.0,
    );
    let right = Vec2::new(
        position.x + (angle + 2.618).cos() * 12.0,
        position.y + (angle + 2.618).sin() * 12.0,
    );
    draw_triangle(nose, left, right, WHITE);
}

fn draw_thrust_flame(position: Vec2, angle: f32) {
    let back = angle + PI;
    let flame_len = rand::gen_range(8.0, 20.0);
    let flame_w = rand::gen_range(3.0, 7.0);

    let base = Vec2::new(position.x + back.cos() * 8.0, position.y + back.sin() * 8.0);
    let tip = Vec2::new(
        base.x + back.cos() * flame_len,
        base.y + back.sin() * flame_len,
    );
    let l = Vec2::new(
        base.x + (back - 1.5).cos() * flame_w,
        base.y + (back - 1.5).sin() * flame_w,
    );
    let r = Vec2::new(
        base.x + (back + 1.5).cos() * flame_w,
        base.y + (back + 1.5).sin() * flame_w,
    );

    draw_triangle(tip, l, r, ORANGE);
    draw_triangle(tip, base, r, RED);
}

fn wrap_position(pos: &mut Vec2) {
    let w = screen_width();
    let h = screen_height();
    if pos.x < 0.0 {
        pos.x += w
    }
    if pos.x > w {
        pos.x -= w
    }
    if pos.y < 0.0 {
        pos.y += h
    }
    if pos.y > h {
        pos.y -= h
    }
}
