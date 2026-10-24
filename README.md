# symmetrical-winner

Rust game leveraging [GLFW](https://www.glfw.org/).

## Prerequisites

- [Rust](https://www.rust-lang.org/) toolchain
- [GLFW](https://www.glfw.org/) and OpenGL development libraries (e.g. `libglfw3-dev` and `libgl1-mesa-dev` on Debian/Ubuntu) so the C FFI wrapper can link against them
- A display server (X11 or Wayland) to run the game window

## Starter

To ensure you are set up correctly, you can run the test C game that comes with this project. You can build and run the test game with:

```bash
cd starter
make run-c
```

## My Game Engine

Test the game engine

```bash
cd starter
make test-rust
```

All of the tests must run individually to verify the correctness of the C FFI bindings.
```bash
cd my_game_engine/
cargo test tests::test_key_presses -- --ignored
cargo test tests::test_screen_clearing -- --ignored
cargo test tests::test_simple_game_loop -- --ignored
cargo test tests::test_sprite_position_update -- --ignored
cargo test tests::test_sprite_rendering -- --ignored
```

* * *

## Rust Test Game

A simple game that showcases the game engine together with multithreading.

The game spawns a random sprite by fetching sprite data from an external API. The HTTP
request is made in a secondary networking thread, so the game loop keeps running smoothly
while the (slow) request is in flight. The fetched sprite data is sent back through a second
message channel and the main thread spawns the sprite.

Run the game with:

```bash
cd rust_test_game
cargo run
```

### Controls

- `SPACE`: Fetch a new random sprite from the API and spawn it.
- `UP` / `DOWN` / `LEFT`: Move the last spawned sprite.
- `RIGHT`: Change the color of the last spawned sprite to green.

* * *

## My Sokoban

A simple Sokoban puzzle game built on top of the game engine.

Run the game with:

```bash
cd my_sokoban
cargo run
```

### Controls

- Arrow keys: Move the player and push boxes.
- Spacebar: Reset the level.

* * *

## Uses

[GLFW](https://www.glfw.org/) is an Open Source, multi-platform library for OpenGL, OpenGL ES and Vulkan development on the desktop.
