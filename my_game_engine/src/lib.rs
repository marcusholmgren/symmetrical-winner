pub mod ffi;

/// Tests for the C FFI bindings
/// All the tests are marked with `#[ignore]` so they don't run in parallel
/// Run them individually to verify the correctness of the C FFI bindings
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_simple_game_loop() {
        let title = b"Test Simple Game Loop\0".as_ptr() as *const ffi::c_char;
        ffi::create_game_window(title, 800, 600);

        while ffi::window_should_close() == 0 {
            ffi::clear_screen();
            ffi::update_game_window();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    #[test]
    #[ignore]
    fn test_sprite_rendering() {
        let title = b"Test Sprite Rendering\0".as_ptr() as *const ffi::c_char;
        ffi::create_game_window(title, 800, 600);

        let sprite = ffi::create_sprite(300.0, 200.0, 200, 200, 255, 0, 0);
        assert_ne!(sprite, std::ptr::null_mut());

        while ffi::window_should_close() == 0 {
            ffi::clear_screen();
            ffi::render_sprite(sprite);
            ffi::update_game_window();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    #[test]
    #[ignore]
    fn test_screen_clearing() {
        let title = b"Test Screen Clearing\0".as_ptr() as *const ffi::c_char;
        ffi::create_game_window(title, 800, 600);

        let red_sprite = ffi::create_sprite(300.0, 200.0, 200, 200, 255, 0, 0);
        let green_sprite = ffi::create_sprite(300.0, 200.0, 200, 200, 0, 255, 0);
        assert_ne!(red_sprite, std::ptr::null_mut());
        assert_ne!(green_sprite, std::ptr::null_mut());

        let start = std::time::Instant::now();

        while ffi::window_should_close() == 0 {
            ffi::clear_screen();
            if start.elapsed() < std::time::Duration::from_secs(5) {
                ffi::render_sprite(red_sprite);
            } else {
                ffi::render_sprite(green_sprite);
            }
            ffi::update_game_window();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    #[test]
    #[ignore]
    fn test_key_presses() {
        let title = b"Test Key Presses - Press SPACE, UP, DOWN, LEFT, RIGHT\0".as_ptr()
            as *const ffi::c_char;
        ffi::create_game_window(title, 800, 600);

        let space_sprite = ffi::create_sprite(100.0, 250.0, 80, 80, 255, 0, 0);
        let up_sprite = ffi::create_sprite(220.0, 250.0, 80, 80, 255, 0, 0);
        let down_sprite = ffi::create_sprite(340.0, 250.0, 80, 80, 255, 0, 0);
        let left_sprite = ffi::create_sprite(460.0, 250.0, 80, 80, 255, 0, 0);
        let right_sprite = ffi::create_sprite(580.0, 250.0, 80, 80, 255, 0, 0);

        let mut space_pressed = false;
        let mut up_pressed = false;
        let mut down_pressed = false;
        let mut left_pressed = false;
        let mut right_pressed = false;

        while ffi::window_should_close() == 0 {
            let win = ffi::get_window();

            if ffi::get_key(win, ffi::GLFW_KEY_SPACE) == ffi::GLFW_PRESS {
                space_pressed = true;
                unsafe {
                    (*space_sprite).color = [0, 255, 0];
                }
            }
            if ffi::get_key(win, ffi::GLFW_KEY_UP) == ffi::GLFW_PRESS {
                up_pressed = true;
                unsafe {
                    (*up_sprite).color = [0, 255, 0];
                }
            }
            if ffi::get_key(win, ffi::GLFW_KEY_DOWN) == ffi::GLFW_PRESS {
                down_pressed = true;
                unsafe {
                    (*down_sprite).color = [0, 255, 0];
                }
            }
            if ffi::get_key(win, ffi::GLFW_KEY_LEFT) == ffi::GLFW_PRESS {
                left_pressed = true;
                unsafe {
                    (*left_sprite).color = [0, 255, 0];
                }
            }
            if ffi::get_key(win, ffi::GLFW_KEY_RIGHT) == ffi::GLFW_PRESS {
                right_pressed = true;
                unsafe {
                    (*right_sprite).color = [0, 255, 0];
                }
            }

            ffi::clear_screen();
            ffi::render_sprite(space_sprite);
            ffi::render_sprite(up_sprite);
            ffi::render_sprite(down_sprite);
            ffi::render_sprite(left_sprite);
            ffi::render_sprite(right_sprite);
            ffi::update_game_window();

            if space_pressed && up_pressed && down_pressed && left_pressed && right_pressed {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    #[test]
    #[ignore]
    fn test_sprite_position_update() {
        let title = b"Test Sprite Position Update\0".as_ptr() as *const ffi::c_char;
        ffi::create_game_window(title, 800, 600);

        let sprite = ffi::create_sprite(0.0, 250.0, 100, 100, 0, 128, 255);
        assert_ne!(sprite, std::ptr::null_mut());

        let mut x = 0.0f32;
        let mut dx = 5.0f32;

        while ffi::window_should_close() == 0 {
            x += dx;
            if x > 700.0 || x < 0.0 {
                dx = -dx;
            }

            ffi::update_sprite_position(sprite, x, 250.0);
            ffi::clear_screen();
            ffi::render_sprite(sprite);
            ffi::update_game_window();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}
