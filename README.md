# Rust Dodge Game

A simple 2D dodge game built with [Macroquad](https://macroquad.rs/) — designed as a hands-on way to learn Rust through game development.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)

## Gameplay

- You control a **green square** at the bottom of the screen
- **Red squares** fall from the top at random speeds
- Dodge them to score points — each enemy that passes off-screen is +1
- Get hit and it's game over! Press **SPACE** or **ENTER** to restart

## Controls

| Key | Action |
|-----|--------|
| `W` / `Up Arrow` | Move up |
| `S` / `Down Arrow` | Move down |
| `A` / `Left Arrow` | Move left |
| `D` / `Right Arrow` | Move right |
| `Space` / `Enter` | Restart (on game over) |

## Setup

### Prerequisites

**Install Rust** (if you haven't already):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**System dependencies** (Arch Linux / i3wm):
```bash
sudo pacman -S base-devel pkg-config libx11 libxi libgl mesa
```

**Ubuntu/Debian:**
```bash
sudo apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev
```

### Run the Game

```bash
# Clone the repo
git clone https://github.com/Tenzubushi/rust-dodge-game.git
cd rust-dodge-game

# Run in debug mode (faster compile, slower game)
cargo run

# Run in release mode (slower compile, smooth gameplay)
cargo run --release
```

## Rust Concepts You'll Learn

The code is heavily commented to explain each Rust concept as it appears:

| Concept | Where in the code |
|---------|-------------------|
| **Structs** | `Player`, `Enemy`, `Game` — grouping related data |
| **Enums** | `GameState` — modeling distinct states |
| **impl blocks** | Methods on structs (like `Player::new()`, `player.update()`) |
| **Vec\<T\>** | `enemies: Vec<Enemy>` — dynamic arrays |
| **Pattern matching** | `match self.state { ... }` — exhaustive branching |
| **Closures** | `.any(\|e\| ...)`, `.retain(\|e\| ...)` — inline functions |
| **Iterators** | `.iter()`, `.iter_mut()` — zero-cost iteration |
| **Ownership & borrowing** | `&self` vs `&mut self` — Rust's core safety model |
| **Constants** | `const PLAYER_SPEED: f32 = 400.0` — compile-time values |
| **Type casting** | `dodged as u32` — explicit conversions |

## Next Steps

Want to keep learning? Try these modifications:

1. **Add a difficulty ramp** — increase `ENEMY_SPEED_MAX` as the score goes up
2. **Add particles** — spawn small particles when an enemy is dodged
3. **Add textures** — replace rectangles with sprites using `load_texture()`
4. **Add sound** — use `macroquad::audio` to play effects
5. **Add a start screen** — add a `GameState::Menu` variant

## License

MIT — do whatever you want with it!
