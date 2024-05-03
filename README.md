# Breakout Game in Rust

![breakoutimage](https://github.com/lostjared/Breakout.Rust/blob/main/capture.jpg "Breakout Image")

## Overview
This project is a Rust implementation of the classic arcade game Breakout. In this game, players control a paddle at the bottom of the screen to bounce a ball against a wall of bricks at the top. The goal is to break all the bricks without letting the ball fall off the bottom of the screen.

## Features
- High-resolution gameplay (1440x1080)
- Randomized brick colors
- Scoring system and live tracking
- Simple and intuitive paddle controls

## Prerequisites
Before you can run this game, you need to have the following installed:
- Rust programming language
- SDL2
- SDL2_ttf (for font rendering)

## Installation
1. **Install Rust:**
   Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install Rust on your machine.

2. **Install SDL2 libraries:**
   SDL2 and SDL2_ttf are required to handle graphics rendering and font management. You can install these libraries through your operating system's package manager. For example, on Ubuntu, you can use:
   ```bash
   sudo apt-get install libsdl2-dev libsdl2-ttf-dev
   ```

## Building and Running
To build and run the game, follow these steps:
1. Clone the repository:
   ```bash
   git clone https://github.com/lostjared/Breakout.Rust.git
   cd Breakout.Rust/breakout-rust
   ```

2. Compile the project:
   ```bash
   cargo build --release
   ```

3. Run the game:
   ```bash
   cargo run --release
   ```

## Controls
- **Left Arrow Key**: Move the paddle left
- **Right Arrow Key**: Move the paddle right
- **Escape Key**: Exit the game

## Contributions
Contributions to this project are welcome. Please fork the repository, make your changes, and submit a pull request.
