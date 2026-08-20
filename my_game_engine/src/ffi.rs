pub use std::os::raw::{c_char, c_float, c_int};

pub const GLFW_PRESS: c_int = 1;
pub const GLFW_RELEASE: c_int = 0;
pub const GLFW_REPEAT: c_int = 2;

pub const GLFW_KEY_SPACE: c_int = 32;
pub const GLFW_KEY_RIGHT: c_int = 262;
pub const GLFW_KEY_LEFT: c_int = 263;
pub const GLFW_KEY_DOWN: c_int = 264;
pub const GLFW_KEY_UP: c_int = 265;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub width: c_int,
    pub height: c_int,
    pub color: [c_int; 3],
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
pub struct GLFWwindow {
    _unused: [u8; 0],
}

mod sys {
    use super::*;

    extern "C" {
        pub fn create_game_window(title: *const c_char, width: c_int, height: c_int);
        pub fn create_sprite(
            x: c_float,
            y: c_float,
            width: c_int,
            height: c_int,
            r: c_int,
            g: c_int,
            b: c_int,
        ) -> *mut Sprite;
        pub fn render_sprite(sprite: *mut Sprite);
        pub fn update_sprite_position(sprite: *mut Sprite, x: c_float, y: c_float);
        pub fn update_game_window();
        pub fn clear_screen();
        pub fn window_should_close() -> c_int;
        pub fn get_key(window: *mut GLFWwindow, key: c_int) -> c_int;
        pub fn get_window() -> *mut GLFWwindow;
    }
}

pub fn create_game_window(title: *const c_char, width: c_int, height: c_int) {
    unsafe {
        sys::create_game_window(title, width, height);
    }
}

pub fn create_sprite(
    x: c_float,
    y: c_float,
    width: c_int,
    height: c_int,
    r: c_int,
    g: c_int,
    b: c_int,
) -> *mut Sprite {
    unsafe { sys::create_sprite(x, y, width, height, r, g, b) }
}

pub fn render_sprite(sprite: *mut Sprite) {
    unsafe {
        sys::render_sprite(sprite);
    }
}

pub fn update_sprite_position(sprite: *mut Sprite, x: c_float, y: c_float) {
    unsafe {
        sys::update_sprite_position(sprite, x, y);
    }
}

pub fn update_game_window() {
    unsafe {
        sys::update_game_window();
    }
}

pub fn clear_screen() {
    unsafe {
        sys::clear_screen();
    }
}

pub fn window_should_close() -> c_int {
    unsafe { sys::window_should_close() }
}

pub fn get_key(window: *mut GLFWwindow, key: c_int) -> c_int {
    unsafe { sys::get_key(window, key) }
}

pub fn get_window() -> *mut GLFWwindow {
    unsafe { sys::get_window() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_struct_layout() {
        assert_eq!(
            std::mem::size_of::<Sprite>(),
            std::mem::size_of::<c_int>() * 5 + std::mem::size_of::<c_float>() * 2
        );
    }
}
