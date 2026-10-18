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


From the root of the project recommended to limit thread to 1

[Running tests consecutively](https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-tests-in-parallel-or-consecutively)

```bash
cd my_game_engine/
cargo test -- --test-threads=1
```

Several of the tests must run individually to verify the correctness of the C FFI bindings.
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
