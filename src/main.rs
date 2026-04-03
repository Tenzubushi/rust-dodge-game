// =============================================================
// Rust Dodge Game — Learn Rust by building a game with Macroquad!
//
// Concepts covered:
//   - Structs and enums
//   - Vectors (Vec<T>)
//   - Pattern matching (match)
//   - Ownership and borrowing
//   - Loops and iterators
//   - Random number generation
//   - Game loop basics
// =============================================================

use macroquad::prelude::*;

// --- Constants ---
// Using constants keeps magic numbers out of your code and makes tuning easy.
const PLAYER_SIZE: f32 = 30.0;
const PLAYER_SPEED: f32 = 400.0;
const ENEMY_SIZE: f32 = 20.0;
const ENEMY_SPEED_MIN: f32 = 100.0;
const ENEMY_SPEED_MAX: f32 = 300.0;
const SPAWN_INTERVAL: f64 = 0.5; // seconds between enemy spawns

// --- Structs ---
// Structs let you group related data together. Think of them like classes
// without inheritance.

/// The player — a rectangle you move with arrow keys or WASD.
struct Player {
    x: f32,
    y: f32,
    size: f32,
}

impl Player {
    /// Create a new player centered at the bottom of the screen.
    fn new() -> Self {
        Self {
            x: screen_width() / 2.0 - PLAYER_SIZE / 2.0,
            y: screen_height() - PLAYER_SIZE - 20.0,
            size: PLAYER_SIZE,
        }
    }

    /// Move the player based on keyboard input.
    /// `dt` (delta time) makes movement frame-rate independent.
    fn update(&mut self, dt: f32) {
        // is_key_down returns true every frame the key is held
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.x -= PLAYER_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.x += PLAYER_SPEED * dt;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.y -= PLAYER_SPEED * dt;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.y += PLAYER_SPEED * dt;
        }

        // Clamp position so the player stays on screen.
        // `.clamp(min, max)` is a handy method on f32.
        self.x = self.x.clamp(0.0, screen_width() - self.size);
        self.y = self.y.clamp(0.0, screen_height() - self.size);
    }

    /// Draw the player as a colored rectangle.
    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.size, self.size, GREEN);
    }

    /// Return the bounding box as a Rect (used for collision detection).
    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.size, self.size)
    }
}

/// An enemy that falls from the top of the screen.
struct Enemy {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
}

impl Enemy {
    /// Spawn an enemy at a random x position along the top.
    fn new() -> Self {
        Self {
            x: rand::gen_range(0.0, screen_width() - ENEMY_SIZE),
            y: -ENEMY_SIZE, // start just above the screen
            size: ENEMY_SIZE,
            speed: rand::gen_range(ENEMY_SPEED_MIN, ENEMY_SPEED_MAX),
        }
    }

    /// Move the enemy downward.
    fn update(&mut self, dt: f32) {
        self.y += self.speed * dt;
    }

    /// Draw the enemy as a red rectangle.
    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.size, self.size, RED);
    }

    /// Return the bounding box for collision checks.
    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.size, self.size)
    }

    /// Has this enemy fallen off the bottom of the screen?
    fn is_off_screen(&self) -> bool {
        self.y > screen_height()
    }
}

// --- Enums ---
// Enums model distinct states. Rust enums are powerful — each variant can
// hold different data (though we keep it simple here).

/// The two states our game can be in.
enum GameState {
    Playing,
    GameOver,
}

/// All the mutable game data, bundled into one struct.
struct Game {
    player: Player,
    enemies: Vec<Enemy>, // Vec<T> is Rust's growable array
    state: GameState,
    score: u32,
    last_spawn_time: f64,
    high_score: u32,
}

impl Game {
    /// Set up a fresh game.
    fn new() -> Self {
        Self {
            player: Player::new(),
            enemies: Vec::new(),
            state: GameState::Playing,
            score: 0,
            last_spawn_time: get_time(),
            high_score: 0,
        }
    }

    /// Reset the game while keeping the high score.
    fn restart(&mut self) {
        self.player = Player::new();
        self.enemies.clear(); // remove all enemies
        self.state = GameState::Playing;
        self.score = 0;
        self.last_spawn_time = get_time();
    }

    /// The main update logic — called once per frame.
    fn update(&mut self) {
        // Pattern matching with `match` — one of Rust's best features!
        match self.state {
            GameState::Playing => self.update_playing(),
            GameState::GameOver => self.update_game_over(),
        }
    }

    fn update_playing(&mut self) {
        let dt = get_frame_time(); // seconds since last frame

        // Update player position
        self.player.update(dt);

        // Spawn new enemies at regular intervals
        let now = get_time();
        if now - self.last_spawn_time >= SPAWN_INTERVAL {
            self.enemies.push(Enemy::new());
            self.last_spawn_time = now;
        }

        // Update all enemies and check for collisions.
        // We iterate mutably over the vec with `.iter_mut()`.
        for enemy in self.enemies.iter_mut() {
            enemy.update(dt);
        }

        // Check collisions — does any enemy overlap the player?
        // `.any()` is an iterator adapter that short-circuits on the first true.
        let player_rect = self.player.rect();
        let hit = self.enemies.iter().any(|e| e.rect().overlaps(&player_rect));

        if hit {
            if self.score > self.high_score {
                self.high_score = self.score;
            }
            self.state = GameState::GameOver;
            return;
        }

        // Remove off-screen enemies and count them as score.
        // `.retain()` keeps only elements where the closure returns true.
        let before = self.enemies.len();
        self.enemies.retain(|e| !e.is_off_screen());
        let dodged = before - self.enemies.len();
        self.score += dodged as u32;
    }

    fn update_game_over(&mut self) {
        // Press SPACE or ENTER to restart
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            self.restart();
        }
    }

    /// Draw everything to the screen.
    fn draw(&self) {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0)); // dark blue-gray

        match self.state {
            GameState::Playing => self.draw_playing(),
            GameState::GameOver => self.draw_game_over(),
        }
    }

    fn draw_playing(&self) {
        // Draw all enemies
        for enemy in &self.enemies {
            enemy.draw();
        }

        // Draw the player
        self.player.draw();

        // Draw the score (HUD)
        draw_text(
            &format!("Score: {}", self.score),
            10.0,
            30.0,
            30.0,
            WHITE,
        );

        if self.high_score > 0 {
            draw_text(
                &format!("Best: {}", self.high_score),
                10.0,
                60.0,
                24.0,
                GRAY,
            );
        }

        // Controls hint
        draw_text(
            "WASD or Arrow Keys to move",
            10.0,
            screen_height() - 10.0,
            20.0,
            DARKGRAY,
        );
    }

    fn draw_game_over(&self) {
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // draw_text positions text by its bottom-left corner,
        // so we offset to roughly center it.
        let title = "GAME OVER";
        let title_size = 60.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(
            title,
            center_x - title_width / 2.0,
            center_y - 40.0,
            title_size,
            RED,
        );

        let score_text = format!("Score: {}   Best: {}", self.score, self.high_score);
        let score_size = 30.0;
        let score_width = measure_text(&score_text, None, score_size as u16, 1.0).width;
        draw_text(
            &score_text,
            center_x - score_width / 2.0,
            center_y + 10.0,
            score_size,
            WHITE,
        );

        let restart_text = "Press SPACE or ENTER to play again";
        let restart_size = 24.0;
        let restart_width = measure_text(restart_text, None, restart_size as u16, 1.0).width;
        draw_text(
            restart_text,
            center_x - restart_width / 2.0,
            center_y + 50.0,
            restart_size,
            YELLOW,
        );
    }
}

// --- Entry Point ---
// Macroquad uses `#[macroquad::main]` instead of the normal `fn main()`.
// This sets up the window and starts the async game loop.

#[macroquad::main("Rust Dodge Game")]
async fn main() {
    let mut game = Game::new();

    // The game loop: runs once per frame, forever.
    loop {
        game.update();
        game.draw();

        // Yield to macroquad so it can render the frame and handle events.
        next_frame().await;
    }
}
