// ---------------------------------------------------------------------------
// spaceship — A standalone asteroids-style ship demo
//
// Controls:
//   ← →  rotate the ship
//   ↑     thrust (with flickering flame)
//   Space shoot (logged only — bullets not yet implemented)
//
// The ship drifts with momentum, wraps around screen edges, and includes
// a simple speed limiter.
// ---------------------------------------------------------------------------

use macroquad::prelude::*;

// ---- tunable physics ------------------------------------------------------

/// How fast the ship spins, in radians per second (~229°/s).
const TURN_RATE: f32 = 4.0;

/// Acceleration applied while thrusting, in pixels/s².
const ENGINE_FORCE: f32 = 300.0;

/// Multiplier applied to velocity each frame — 1.0 = no drag, 0.0 = stop.
const HULL_DRAG: f32 = 0.98;

/// Any velocity above this gets clamped back to this value.
const SPEED_CAP: f32 = 400.0;

/// Half-width of the ship triangle base.
const HULL_BASE: f32 = 12.0;

/// Distance from centre to nose tip.
const HULL_NOSE: f32 = 20.0;

/// Wing sweep from the nose direction, in radians (~150°).
const SWEEP: f32 = 2.618;

// ---- ship state -----------------------------------------------------------

struct Spaceship {
    pos: Vec2,             // centre of the ship on screen
    vel: Vec2,             // current velocity vector
    heading: f32,          // direction the nose points (radians, 0 = right)
    burning: bool,         // true while the thruster key is held
    dead: bool,            // true after an asteroid hit (not implemented yet)
}

impl Spaceship {
    /// Create a new ship at the centre of the screen, stationary.
    fn spawn() -> Self {
        Self {
            pos: Vec2::new(screen_width() * 0.5, screen_height() * 0.5),
            vel: Vec2::ZERO,
            heading: 0.0,
            burning: false,
            dead: false,
        }
    }

    /// Advance the ship by one frame.
    fn tick(&mut self, dt: f32, input: &Controls) {
        if self.dead {
            return;
        }

        // --- rotation ------------------------------------------------------
        if input.turn_left {
            self.heading -= TURN_RATE * dt;
        }
        if input.turn_right {
            self.heading += TURN_RATE * dt;
        }

        // --- thrust --------------------------------------------------------
        self.burning = input.thrust;
        if self.burning {
            // Push in the direction the nose points.
            self.vel.x += self.heading.cos() * ENGINE_FORCE * dt;
            self.vel.y += self.heading.sin() * ENGINE_FORCE * dt;
        }

        // --- drag & speed limit --------------------------------------------
        self.vel *= HULL_DRAG;
        let speed = self.vel.length();
        if speed > SPEED_CAP {
            self.vel = self.vel.normalize_or_zero() * SPEED_CAP;
        }

        // --- move & wrap ---------------------------------------------------
        self.pos += self.vel * dt;
        wrap_around(&mut self.pos);
    }

    /// Draw the ship (and its flame) onto the screen.
    fn render(&self) {
        if self.dead {
            return;
        }

        // Flame is drawn *behind* the hull so it stays under the triangle.
        if self.burning {
            draw_exhaust(self.pos, self.heading);
        }

        draw_hull(self.pos, self.heading);
    }
}

// ---- input helper ---------------------------------------------------------

/// Snapshots of the four buttons we care about each frame.
struct Controls {
    turn_left: bool,
    turn_right: bool,
    thrust: bool,
    pew: bool, // single-press shot (not wired to anything yet)
}

/// Poll the keyboard and return a clean input struct.
fn read_controls() -> Controls {
    Controls {
        turn_left:  is_key_down(KeyCode::Left),
        turn_right: is_key_down(KeyCode::Right),
        thrust:     is_key_down(KeyCode::Up),
        pew:        is_key_pressed(KeyCode::Space),
    }
}

// ---- drawing helpers ------------------------------------------------------

/// Draw the ship body as a triangle: one nose, two wings.
fn draw_hull(centre: Vec2, nose_angle: f32) {
    // Tip — straight ahead from centre.
    let tip = Vec2::new(
        centre.x + nose_angle.cos() * HULL_NOSE,
        centre.y + nose_angle.sin() * HULL_NOSE,
    );

    // Port (left) wing — sweep counter-clockwise from the nose.
    let port = Vec2::new(
        centre.x + (nose_angle - SWEEP).cos() * HULL_BASE,
        centre.y + (nose_angle - SWEEP).sin() * HULL_BASE,
    );

    // Starboard (right) wing — sweep clockwise (Y points down).
    let starboard = Vec2::new(
        centre.x + (nose_angle + SWEEP).cos() * HULL_BASE,
        centre.y + (nose_angle + SWEEP).sin() * HULL_BASE,
    );

    draw_triangle(tip, port, starboard, WHITE);
}

/// Flickering orange exhaust triangle that shoots out the back.
fn draw_exhaust(centre: Vec2, nose_angle: f32) {
    // "Back" is exactly opposite the nose.
    let tail = nose_angle + std::f32::consts::PI;

    // Random flicker — length and width vary every frame.
    let length = rand::gen_range(8.0, 20.0);
    let spread = rand::gen_range(3.0, 7.0);

    // Base of the flame, a little behind centre.
    let base = Vec2::new(
        centre.x + tail.cos() * 8.0,
        centre.y + tail.sin() * 8.0,
    );

    // Tip of the flame, further back by `length`.
    let tip = Vec2::new(
        base.x + tail.cos() * length,
        base.y + tail.sin() * length,
    );

    // Left and right edges of the flame triangle.
    let left = Vec2::new(
        base.x + (tail + 1.5).cos() * spread,
        base.y + (tail + 1.5).sin() * spread,
    );
    let right = Vec2::new(
        base.x + (tail - 1.5).cos() * spread,
        base.y + (tail - 1.5).sin() * spread,
    );

    draw_triangle(tip, left, right, ORANGE);
    draw_triangle(tip, base, right, RED); // darker inner core
}

/// Teleport the ship to the opposite edge when it goes off-screen.
fn wrap_around(pos: &mut Vec2) {
    let w = screen_width();
    let h = screen_height();
    if pos.x < 0.0 {
        pos.x += w;
    }
    if pos.x > w {
        pos.x -= w;
    }
    if pos.y < 0.0 {
        pos.y += h;
    }
    if pos.y > h {
        pos.y -= h;
    }
}

// ---- entry point ----------------------------------------------------------

#[macroquad::main("Spaceship")]
async fn main() {
    let mut ship = Spaceship::spawn();

    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        // Read inputs once and share across all systems.
        let controls = read_controls();

        ship.tick(dt, &controls);

        // Log shoot for later (Chapter 6+).
        if controls.pew {
            println!("pew!"); // placeholder
        }

        ship.render();

        next_frame().await;
    }
}
