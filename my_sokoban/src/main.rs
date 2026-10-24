use my_game_engine::*;

const TILE_SIZE: f32 = 64.0;
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

const LEVEL_1: &str = "\
######
#    #
#P B #
# T  #
######";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    Target,
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub sprite: *mut ffi::Sprite,
}

pub struct BoxEntity {
    pub x: i32,
    pub y: i32,
    pub sprite: *mut ffi::Sprite,
}

pub struct SokobanGame {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>,
    pub targets: Vec<(i32, i32)>,
    pub player: Player,
    pub boxes: Vec<BoxEntity>,
    pub is_level_complete: bool,

    // Centering offsets
    pub offset_x: f32,
    pub offset_y: f32,

    // Static floor, wall, and target sprites for rendering
    pub floors: Vec<*mut ffi::Sprite>,
    pub target_sprites: Vec<*mut ffi::Sprite>,
    pub walls: Vec<*mut ffi::Sprite>,
}

impl SokobanGame {
    pub fn new() -> Self {
        let lines: Vec<&str> = LEVEL_1.lines().filter(|l| !l.is_empty()).collect();
        let height = lines.len() as i32;
        let width = if height > 0 { lines[0].len() as i32 } else { 0 };

        let offset_x = (SCREEN_WIDTH as f32 - (width as f32 * TILE_SIZE)) / 2.0;
        let offset_y = (SCREEN_HEIGHT as f32 - (height as f32 * TILE_SIZE)) / 2.0;

        let mut tiles = vec![vec![Tile::Floor; width as usize]; height as usize];
        let mut player_pos = (0, 0);
        let mut box_positions = Vec::new();
        let mut targets = Vec::new();

        for y in 0..height {
            let line = lines[y as usize];
            let chars: Vec<char> = line.chars().collect();
            for x in 0..width {
                let ch = chars[x as usize];
                match ch {
                    '#' => {
                        tiles[y as usize][x as usize] = Tile::Wall;
                    }
                    'T' => {
                        tiles[y as usize][x as usize] = Tile::Target;
                        targets.push((x, y));
                    }
                    'P' => {
                        tiles[y as usize][x as usize] = Tile::Floor;
                        player_pos = (x, y);
                    }
                    'B' => {
                        tiles[y as usize][x as usize] = Tile::Floor;
                        box_positions.push((x, y));
                    }
                    _ => {}
                }
            }
        }

        let player = Player {
            x: player_pos.0,
            y: player_pos.1,
            sprite: std::ptr::null_mut(),
        };

        let boxes = box_positions
            .into_iter()
            .map(|(bx, by)| BoxEntity {
                x: bx,
                y: by,
                sprite: std::ptr::null_mut(),
            })
            .collect();

        Self {
            width,
            height,
            tiles,
            targets,
            player,
            boxes,
            is_level_complete: false,
            offset_x,
            offset_y,
            floors: Vec::new(),
            target_sprites: Vec::new(),
            walls: Vec::new(),
        }
    }

    pub fn init_sprites(&mut self) {
        self.floors.clear();
        self.target_sprites.clear();
        self.walls.clear();

        // 1. Spawn floors for all grid cells
        for y in 0..self.height {
            for x in 0..self.width {
                let sx = self.offset_x + (x as f32) * TILE_SIZE;
                let sy = self.offset_y + (y as f32) * TILE_SIZE;
                let sprite = spawn_sprite!(sx, sy, TILE_SIZE, TILE_SIZE, 26, 26, 36);
                self.floors.push(sprite);
            }
        }

        // 2. Spawn target markers (pink squares)
        for &(tx, ty) in &self.targets {
            let sx = self.offset_x + (tx as f32) * TILE_SIZE;
            let sy = self.offset_y + (ty as f32) * TILE_SIZE;
            let size = TILE_SIZE - 24.0;
            let offset = 12.0;
            let sprite = spawn_sprite!(sx + offset, sy + offset, size, size, 232, 67, 147);
            self.target_sprites.push(sprite);
        }

        // 3. Spawn walls (slate blue)
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y as usize][x as usize] == Tile::Wall {
                    let sx = self.offset_x + (x as f32) * TILE_SIZE;
                    let sy = self.offset_y + (y as f32) * TILE_SIZE;
                    let sprite = spawn_sprite!(sx, sy, TILE_SIZE, TILE_SIZE, 44, 62, 80);
                    self.walls.push(sprite);
                }
            }
        }

        // 4. Spawn boxes
        for b in &mut self.boxes {
            let sx = self.offset_x + (b.x as f32) * TILE_SIZE;
            let sy = self.offset_y + (b.y as f32) * TILE_SIZE;
            let on_target = self.targets.contains(&(b.x, b.y));
            let color = if on_target {
                (46, 204, 113)
            } else {
                (243, 156, 18)
            };
            b.sprite = spawn_sprite!(sx, sy, TILE_SIZE, TILE_SIZE, color.0, color.1, color.2);
        }

        // 5. Spawn player
        let px = self.offset_x + (self.player.x as f32) * TILE_SIZE;
        let py = self.offset_y + (self.player.y as f32) * TILE_SIZE;
        self.player.sprite = spawn_sprite!(px, py, TILE_SIZE, TILE_SIZE, 0, 206, 201);
    }

    pub fn reset(&mut self) {
        let lines: Vec<&str> = LEVEL_1.lines().filter(|l| !l.is_empty()).collect();
        let mut player_pos = (0, 0);
        let mut box_idx = 0;

        for y in 0..self.height {
            let line = lines[y as usize];
            let chars: Vec<char> = line.chars().collect();
            for x in 0..self.width {
                let ch = chars[x as usize];
                match ch {
                    'P' => player_pos = (x, y),
                    'B' => {
                        if box_idx < self.boxes.len() {
                            self.boxes[box_idx].x = x;
                            self.boxes[box_idx].y = y;
                            box_idx += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        self.player.x = player_pos.0;
        self.player.y = player_pos.1;
        self.is_level_complete = false;

        // Reset player sprite color to Cyan
        change_sprite_color!(self.player.sprite, 0, 206, 201);

        self.update_sprites();
    }

    pub fn update_sprites(&self) {
        // Move player sprite
        let px = self.offset_x + (self.player.x as f32) * TILE_SIZE;
        let py = self.offset_y + (self.player.y as f32) * TILE_SIZE;
        move_sprite!(self.player.sprite, px, py);

        // Move box sprites and update their colors
        for b in &self.boxes {
            let bx = self.offset_x + (b.x as f32) * TILE_SIZE;
            let by = self.offset_y + (b.y as f32) * TILE_SIZE;
            move_sprite!(b.sprite, bx, by);

            let on_target = self.targets.contains(&(b.x, b.y));
            if on_target {
                change_sprite_color!(b.sprite, 46, 204, 113); // Green
            } else {
                change_sprite_color!(b.sprite, 243, 156, 18); // Gold/Orange
            }
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        if self.is_level_complete {
            return;
        }

        let target_x = self.player.x + dx;
        let target_y = self.player.y + dy;

        // Boundary check
        if target_x < 0 || target_x >= self.width || target_y < 0 || target_y >= self.height {
            return;
        }

        // Wall check
        if self.tiles[target_y as usize][target_x as usize] == Tile::Wall {
            return;
        }

        // Check if there is a box at the target position
        let mut box_idx = None;
        for (i, b) in self.boxes.iter().enumerate() {
            if b.x == target_x && b.y == target_y {
                box_idx = Some(i);
                break;
            }
        }

        if let Some(i) = box_idx {
            // Box pushing logic: check tile beyond the box
            let beyond_x = target_x + dx;
            let beyond_y = target_y + dy;

            // Boundary check beyond
            if beyond_x < 0 || beyond_x >= self.width || beyond_y < 0 || beyond_y >= self.height {
                return;
            }

            // Wall check beyond
            if self.tiles[beyond_y as usize][beyond_x as usize] == Tile::Wall {
                return;
            }

            // Box check beyond
            let box_beyond = self
                .boxes
                .iter()
                .any(|b| b.x == beyond_x && b.y == beyond_y);
            if box_beyond {
                return;
            }

            // Advance box and player
            self.boxes[i].x = beyond_x;
            self.boxes[i].y = beyond_y;
            self.player.x = target_x;
            self.player.y = target_y;
            println!("[Sokoban] Pushed Box to ({}, {})", beyond_x, beyond_y);
        } else {
            // Move player to target
            self.player.x = target_x;
            self.player.y = target_y;
        }

        // Re-render positions
        self.update_sprites();

        // Check win condition
        self.check_win();
    }

    pub fn check_win(&mut self) {
        let win = self
            .targets
            .iter()
            .all(|&(tx, ty)| self.boxes.iter().any(|b| b.x == tx && b.y == ty));

        if win {
            self.is_level_complete = true;
            println!("======================================");
            println!("[Sokoban] Level Complete! You Solved It!");
            println!("Press SPACEBAR to reset and play again.");
            println!("======================================");

            // Turn player purple as a celebratory indicator
            change_sprite_color!(self.player.sprite, 155, 89, 182);
        }
    }

    pub fn render(&self) {
        // Redraw floors
        for &s in &self.floors {
            ffi::render_sprite(s);
        }
        // Redraw targets
        for &s in &self.target_sprites {
            ffi::render_sprite(s);
        }
        // Redraw walls
        for &s in &self.walls {
            ffi::render_sprite(s);
        }
        // Redraw boxes
        for b in &self.boxes {
            ffi::render_sprite(b.sprite);
        }
        // Redraw player
        ffi::render_sprite(self.player.sprite);
    }
}

// Debounce helper functions for arrow keys
fn trigger_move_up(game: &mut SokobanGame, up_held: &mut bool) {
    if !*up_held {
        game.move_player(0, -1);
        *up_held = true;
    }
}

fn trigger_move_down(game: &mut SokobanGame, down_held: &mut bool) {
    if !*down_held {
        game.move_player(0, 1);
        *down_held = true;
    }
}

fn trigger_move_left(game: &mut SokobanGame, left_held: &mut bool) {
    if !*left_held {
        game.move_player(-1, 0);
        *left_held = true;
    }
}

fn trigger_move_right(game: &mut SokobanGame, right_held: &mut bool) {
    if !*right_held {
        game.move_player(1, 0);
        *right_held = true;
    }
}

fn main() {
    let mut game = SokobanGame::new();

    let mut space_held = false;
    let mut up_held = false;
    let mut down_held = false;
    let mut left_held = false;
    let mut right_held = false;

    println!("Starting Sokoban!");
    println!("Controls:");
    println!("  - Arrow keys: Move player & push box");
    println!("  - Spacebar: Reset level");

    start_window_and_game_loop! {
        title: "Sokoban Puzzle Game",
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        on_start: {
            game.init_sprites();
        },
        on_tick: {
            let win = ffi::get_window();

            // 1. on_key_press!(key, block) -> Space bar resets level
            on_key_press!(ffi::GLFW_KEY_SPACE, {
                if !space_held {
                    println!("[Sokoban] Resetting Level...");
                    game.reset();
                    space_held = true;
                }
            });

            // 2. on_key_press!(key, expr) -> UP key moves player up
            on_key_press!(
                ffi::GLFW_KEY_UP,
                trigger_move_up(&mut game, &mut up_held)
            );

            // 3. on_key_press!(win, key, block) -> DOWN key moves player down
            on_key_press!(win, ffi::GLFW_KEY_DOWN, {
                trigger_move_down(&mut game, &mut down_held);
            });

            // 4. on_key_press!(win, key, expr) -> LEFT key moves player left
            on_key_press!(
                win,
                ffi::GLFW_KEY_LEFT,
                trigger_move_left(&mut game, &mut left_held)
            );

            // Right key uses the window macro form with a block
            on_key_press!(win, ffi::GLFW_KEY_RIGHT, {
                trigger_move_right(&mut game, &mut right_held);
            });

            // Update debounce states
            if ffi::get_key(win, ffi::GLFW_KEY_SPACE) != ffi::GLFW_PRESS {
                space_held = false;
            }
            if ffi::get_key(win, ffi::GLFW_KEY_UP) != ffi::GLFW_PRESS {
                up_held = false;
            }
            if ffi::get_key(win, ffi::GLFW_KEY_DOWN) != ffi::GLFW_PRESS {
                down_held = false;
            }
            if ffi::get_key(win, ffi::GLFW_KEY_LEFT) != ffi::GLFW_PRESS {
                left_held = false;
            }
            if ffi::get_key(win, ffi::GLFW_KEY_RIGHT) != ffi::GLFW_PRESS {
                right_held = false;
            }

            // Rendering pipeline
            ffi::clear_screen();
            game.render();
        },
        on_exit: {
            println!("[Sokoban] Game exiting. Thanks for playing!");
        }
    }
}
