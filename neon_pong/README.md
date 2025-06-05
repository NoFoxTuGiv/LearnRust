# Neon Pong by Aaron Chavez (2025)

A simple Pong clone built in Rust using [Macroquad](https://github.com/not-fl3/macroquad).  
Control a paddle on the left side, face off against a basic AI on the right, and keep score as the ball bounces back and forth.

---

## Features

- Ball physics with top/bottom wall bounces  
- Debounced paddle collisions for realistic gameplay  
- Simple AI that follows the ball’s Y position  
- Immediate scoring and ball reset when it leaves the left or right edge  
- Displays player and AI scores in real time  

---

## Getting Started

### Prerequisites

- Install Rust and Cargo (Rust 1.65 or later recommended)  
- Clone this repo:
  ```bash
  git clone https://github.com/nofoxtugiv/neon-pong.git
  cd neon-pong
- Ensure you ahve a working GPU and SDL2-compatible drivers.

### Build & Run

- From the project root, run:
```bash
cargo run --release
```
- This will compile the crate and launch the game window. Exit by pressing Escape.

### Controls
- W: Move player paddle up
- S: Move player paddle down
- Escape: Quit the game
- The AI paddle on the right side moves automatically to track the ball.

