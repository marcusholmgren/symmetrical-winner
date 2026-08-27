/// Macro around `update_game_window` and `std::thread::sleep` meant to be invoked at the end of each game loop iteration.
#[macro_export]
macro_rules! tick {
    () => {
        $crate::tick!(16);
    };
    ($millis:expr) => {
        $crate::ffi::update_game_window();
        ::std::thread::sleep(::std::time::Duration::from_millis($millis as u64));
    };
}

/// Macro around bindings to `create_sprite` and `render_sprite`.
#[macro_export]
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $width:expr, $height:expr, $r:expr, $g:expr, $b:expr) => {{
        let sprite = $crate::ffi::create_sprite(
            $x as f32,
            $y as f32,
            $width as i32,
            $height as i32,
            $r as i32,
            $g as i32,
            $b as i32,
        );
        $crate::ffi::render_sprite(sprite);
        sprite
    }};
}

/// Macro around bindings to `get_key` leveraging `GLFW_PRESS` and performing an action when a key is pressed.
#[macro_export]
macro_rules! on_key_press {
    ($key:expr, $action:block) => {
        if $crate::ffi::get_key($crate::ffi::get_window(), $key) == $crate::ffi::GLFW_PRESS $action
    };
    ($key:expr, $action:expr) => {
        if $crate::ffi::get_key($crate::ffi::get_window(), $key) == $crate::ffi::GLFW_PRESS {
            $action;
        }
    };
    ($win:expr, $key:expr, $action:block) => {
        if $crate::ffi::get_key($win, $key) == $crate::ffi::GLFW_PRESS $action
    };
    ($win:expr, $key:expr, $action:expr) => {
        if $crate::ffi::get_key($win, $key) == $crate::ffi::GLFW_PRESS {
            $action;
        }
    };
}

/// Macro around `update_sprite_position` together with `render_sprite` (and optionally `clear_screen`).
#[macro_export]
macro_rules! move_sprite {
    ($sprite:expr, $x:expr, $y:expr) => {{
        $crate::ffi::update_sprite_position($sprite, $x as f32, $y as f32);
        $crate::ffi::render_sprite($sprite);
    }};
    (clear, $sprite:expr, $x:expr, $y:expr) => {{
        $crate::ffi::clear_screen();
        $crate::ffi::update_sprite_position($sprite, $x as f32, $y as f32);
        $crate::ffi::render_sprite($sprite);
    }};
}

/// Macro to change a sprite's color.
#[macro_export]
macro_rules! change_sprite_color {
    ($sprite:expr, $r:expr, $g:expr, $b:expr) => {
        unsafe {
            (*$sprite).color = [$r as i32, $g as i32, $b as i32];
        }
    };
}

/// Macro around `create_game_window` that creates a loop over `window_should_close`
/// calling `tick!` at the end of each iteration.
#[macro_export]
macro_rules! start_window_and_game_loop {
    ($title:expr, $width:expr, $height:expr, $body:block) => {{
        let c_title = ::std::ffi::CString::new($title).expect("CString::new failed");
        $crate::ffi::create_game_window(c_title.as_ptr(), $width as i32, $height as i32);
        while $crate::ffi::window_should_close() == 0 {
            $body
            $crate::tick!();
        }
    }};
    ($title:expr, $width:expr, $height:expr, $on_start:block, $on_tick:block, $on_exit:block) => {{
        let c_title = ::std::ffi::CString::new($title).expect("CString::new failed");
        $crate::ffi::create_game_window(c_title.as_ptr(), $width as i32, $height as i32);
        $on_start
        while $crate::ffi::window_should_close() == 0 {
            $on_tick
            $crate::tick!();
        }
        $on_exit
    }};
    (
        title: $title:expr,
        width: $width:expr,
        height: $height:expr,
        on_start: $on_start:block,
        on_tick: $on_tick:block,
        on_exit: $on_exit:block
    ) => {{
        let c_title = ::std::ffi::CString::new($title).expect("CString::new failed");
        $crate::ffi::create_game_window(c_title.as_ptr(), $width as i32, $height as i32);
        $on_start
        while $crate::ffi::window_should_close() == 0 {
            $on_tick
            $crate::tick!();
        }
        $on_exit
    }};
    (
        title: $title:expr,
        width: $width:expr,
        height: $height:expr,
        on_tick: $on_tick:block
    ) => {{
        let c_title = ::std::ffi::CString::new($title).expect("CString::new failed");
        $crate::ffi::create_game_window(c_title.as_ptr(), $width as i32, $height as i32);
        while $crate::ffi::window_should_close() == 0 {
            $on_tick
            $crate::tick!();
        }
    }};
    (
        title: $title:expr,
        width: $width:expr,
        height: $height:expr,
        on_start: $on_start:block,
        on_tick: $on_tick:block
    ) => {{
        let c_title = ::std::ffi::CString::new($title).expect("CString::new failed");
        $crate::ffi::create_game_window(c_title.as_ptr(), $width as i32, $height as i32);
        $on_start
        while $crate::ffi::window_should_close() == 0 {
            $on_tick
            $crate::tick!();
        }
    }};
}
