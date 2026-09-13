# symmetrical-winner

Rust game leveraging [GLFW](https://www.glfw.org/).

## Starter

To ensure you are set up correctly, you can run the test C game that comes with this project. You can build and run the test game with:

```bash
cd intro-to-rust-starter/starter
make run-c
```

## My Game Engine

Test the game engine

```bash
cd intro-to-rust-starter/starter
make test-rust
```


From the root of the project
```bash
cd my_game_engine/
cargo test
```

Several of the tests are marked with `#[ignore]` so they don't run in parallel. Run them individually to verify the correctness of the C FFI bindings.
```bash
cargo test tests::test_key_presses
cargo test tests::test_screen_clearing
cargo test tests::test_simple_game_loop
cargo test tests::test_sprite_position_update
cargo test tests::test_sprite_rendering
```

* * *

## Uses

[GLFW](https://www.glfw.org/) is an Open Source, multi-platform library for OpenGL, OpenGL ES and Vulkan development on the desktop.
