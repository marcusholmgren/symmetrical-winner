pub mod ffi;
pub mod macros;

/// Tests for the C FFI bindings
/// All the tests are marked with `#[ignore]` so they don't run in parallel
/// Run them individually to verify the correctness of the C FFI bindings
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_simple_game_loop() {
        start_window_and_game_loop! {
            title: "Test Simple Game Loop",
            width: 800,
            height: 600,
            on_tick: {
                ffi::clear_screen();
            }
        }
    }

    #[test]
    #[ignore]
    fn test_sprite_rendering() {
        let sprite;

        start_window_and_game_loop! {
            title: "Test Sprite Rendering",
            width: 800,
            height: 600,
            on_start: {
                sprite = spawn_sprite!(300, 200, 200, 200, 255, 0, 0);
                assert_ne!(sprite, std::ptr::null_mut());
            },
            on_tick: {
                ffi::clear_screen();
                ffi::render_sprite(sprite);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_screen_clearing() {
        let red_sprite;
        let green_sprite;
        let start;

        start_window_and_game_loop! {
            title: "Test Screen Clearing",
            width: 800,
            height: 600,
            on_start: {
                red_sprite = spawn_sprite!(300, 200, 200, 200, 255, 0, 0);
                green_sprite = spawn_sprite!(300, 200, 200, 200, 0, 255, 0);
                assert_ne!(red_sprite, std::ptr::null_mut());
                assert_ne!(green_sprite, std::ptr::null_mut());
                start = std::time::Instant::now();
            },
            on_tick: {
                ffi::clear_screen();
                if start.elapsed() < std::time::Duration::from_secs(5) {
                    ffi::render_sprite(red_sprite);
                } else {
                    ffi::render_sprite(green_sprite);
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn test_key_presses() {
        let space_sprite;
        let up_sprite;
        let down_sprite;
        let left_sprite;
        let right_sprite;

        let mut space_pressed = false;
        let mut up_pressed = false;
        let mut down_pressed = false;
        let mut left_pressed = false;
        let mut right_pressed = false;

        start_window_and_game_loop! {
            title: "Test Key Presses - Press SPACE, UP, DOWN, LEFT, RIGHT",
            width: 800,
            height: 600,
            on_start: {
                space_sprite = spawn_sprite!(100, 250, 80, 80, 255, 0, 0);
                up_sprite = spawn_sprite!(220, 250, 80, 80, 255, 0, 0);
                down_sprite = spawn_sprite!(340, 250, 80, 80, 255, 0, 0);
                left_sprite = spawn_sprite!(460, 250, 80, 80, 255, 0, 0);
                right_sprite = spawn_sprite!(580, 250, 80, 80, 255, 0, 0);
            },
            on_tick: {
                on_key_press!(ffi::GLFW_KEY_SPACE, {
                    space_pressed = true;
                    change_sprite_color!(space_sprite, 0, 255, 0);
                });
                on_key_press!(ffi::GLFW_KEY_UP, {
                    up_pressed = true;
                    change_sprite_color!(up_sprite, 0, 255, 0);
                });
                on_key_press!(ffi::GLFW_KEY_DOWN, {
                    down_pressed = true;
                    change_sprite_color!(down_sprite, 0, 255, 0);
                });
                on_key_press!(ffi::GLFW_KEY_LEFT, {
                    left_pressed = true;
                    change_sprite_color!(left_sprite, 0, 255, 0);
                });
                on_key_press!(ffi::GLFW_KEY_RIGHT, {
                    right_pressed = true;
                    change_sprite_color!(right_sprite, 0, 255, 0);
                });

                ffi::clear_screen();
                ffi::render_sprite(space_sprite);
                ffi::render_sprite(up_sprite);
                ffi::render_sprite(down_sprite);
                ffi::render_sprite(left_sprite);
                ffi::render_sprite(right_sprite);

                if space_pressed && up_pressed && down_pressed && left_pressed && right_pressed {
                    break;
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn test_sprite_position_update() {
        let sprite;
        let mut x = 0.0f32;
        let mut dx = 5.0f32;

        start_window_and_game_loop! {
            title: "Test Sprite Position Update",
            width: 800,
            height: 600,
            on_start: {
                sprite = spawn_sprite!(0, 250, 100, 100, 0, 128, 255);
                assert_ne!(sprite, std::ptr::null_mut());
            },
            on_tick: {
                x += dx;
                if x > 700.0 || x < 0.0 {
                    dx = -dx;
                }
                move_sprite!(clear, sprite, x, 250.0);
            }
        }
    }
}
