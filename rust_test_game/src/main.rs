use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use serde::{Deserialize, Serialize};
use my_game_engine::*;

/// Structure representing sprite data returned from the API endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteData {
    pub b: i64,
    pub g: i64,
    pub height: i64,
    pub r: i64,
    pub width: i64,
    pub x: i64,
    pub y: i64,
}

/// Commands sent from the main thread to the networking thread
pub enum NetworkCommand {
    FetchSprite,
    Quit,
}

/// Custom sender wrapper for sending commands to the networking thread
pub struct MainToNetSender {
    pub inner: Sender<NetworkCommand>,
}

impl MainToNetSender {
    pub fn send(&self, cmd: NetworkCommand) -> Result<(), std::sync::mpsc::SendError<NetworkCommand>> {
        self.inner.send(cmd)
    }
}

/// Custom receiver wrapper for receiving commands in the networking thread
pub struct MainToNetReceiver {
    pub inner: Receiver<NetworkCommand>,
}

impl MainToNetReceiver {
    pub fn recv(&self) -> Result<NetworkCommand, std::sync::mpsc::RecvError> {
        self.inner.recv()
    }
}

/// Custom sender wrapper for sending SpriteData back to the main thread
pub struct NetToMainSender {
    pub inner: Sender<SpriteData>,
}

impl NetToMainSender {
    pub fn send(&self, data: SpriteData) -> Result<(), std::sync::mpsc::SendError<SpriteData>> {
        self.inner.send(data)
    }
}

/// Custom receiver wrapper for receiving SpriteData in the main thread
pub struct NetToMainReceiver {
    pub inner: Receiver<SpriteData>,
}

impl NetToMainReceiver {
    pub fn try_recv(&self) -> Result<SpriteData, std::sync::mpsc::TryRecvError> {
        self.inner.try_recv()
    }
}

/// Worker function for the networking thread
fn networking_thread_worker(cmd_rx: MainToNetReceiver, data_tx: NetToMainSender) {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_else(|_| reqwest::blocking::Client::new());
    let url = "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler";

    while let Ok(cmd) = cmd_rx.recv() {
        match cmd {
            NetworkCommand::FetchSprite => {
                println!("[NetworkingThread] Fetching sprite data from API...");
                match client.get(url).send() {
                    Ok(response) => {
                        match response.json::<SpriteData>() {
                            Ok(sprite_data) => {
                                println!("[NetworkingThread] Received sprite data: {:?}", sprite_data);
                                let _ = data_tx.send(sprite_data);
                            }
                            Err(err) => {
                                eprintln!("[NetworkingThread] Failed to parse JSON response: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("[NetworkingThread] HTTP request error: {}", err);
                    }
                }
            }
            NetworkCommand::Quit => {
                println!("[NetworkingThread] Received Quit command. Exiting thread.");
                break;
            }
        }
    }
}

fn main() {
    // Setup message passing channels with custom wrapper structs
    let (main_tx, net_rx) = channel::<NetworkCommand>();
    let (net_tx, main_rx) = channel::<SpriteData>();

    let main_to_net_tx = MainToNetSender { inner: main_tx };
    let main_to_net_rx = MainToNetReceiver { inner: net_rx };

    let net_to_main_tx = NetToMainSender { inner: net_tx };
    let net_to_main_rx = NetToMainReceiver { inner: main_rx };

    // Spawn the secondary networking thread
    let mut networking_thread_handle: Option<JoinHandle<()>> = Some(thread::spawn(move || {
        networking_thread_worker(main_to_net_rx, net_to_main_tx);
    }));

    let mut spawned_sprites: Vec<*mut ffi::Sprite> = Vec::new();
    let mut space_held = false;

    println!("Starting Game Engine!");
    println!("Controls:");
    println!("  - SPACE: Fetch and spawn a new sprite");
    println!("  - UP / DOWN / LEFT: Move the last spawned sprite");
    println!("  - RIGHT: Change the color of the last spawned sprite to Green");

    start_window_and_game_loop! {
        title: "Rust Test Game - Keyboard Macros & Multithreading",
        width: 800,
        height: 600,
        on_start: {
            // Initial instructions / setup
        },
        on_tick: {
            let win = ffi::get_window();

            // Retrieve the last spawned sprite if any
            let last_sprite_opt = spawned_sprites.last().cloned();

            // 1. on_key_press!(key, block) -> Space bar triggers sprite fetch
            on_key_press!(ffi::GLFW_KEY_SPACE, {
                if !space_held {
                    println!("[MainThread] Key press detected. Requesting new sprite data...");
                    let _ = main_to_net_tx.send(NetworkCommand::FetchSprite);
                    space_held = true;
                }
            });

            if ffi::get_key(win, ffi::GLFW_KEY_SPACE) != ffi::GLFW_PRESS {
                space_held = false;
            }

            if let Some(last_sprite) = last_sprite_opt {
                unsafe {
                    // 2. on_key_press!(key, expr) -> UP key moves last sprite up
                    on_key_press!(
                        ffi::GLFW_KEY_UP,
                        move_sprite!(last_sprite, (*last_sprite).x, (*last_sprite).y + 5.0)
                    );

                    // 3. on_key_press!(win, key, block) -> DOWN key moves last sprite down
                    on_key_press!(win, ffi::GLFW_KEY_DOWN, {
                        move_sprite!(last_sprite, (*last_sprite).x, (*last_sprite).y - 5.0);
                    });

                    // 4. on_key_press!(win, key, expr) -> LEFT key moves last sprite left
                    on_key_press!(
                        win,
                        ffi::GLFW_KEY_LEFT,
                        move_sprite!(last_sprite, (*last_sprite).x - 5.0, (*last_sprite).y)
                    );

                    // Also change sprite color using change_sprite_color! on RIGHT key press
                    on_key_press!(win, ffi::GLFW_KEY_RIGHT, {
                        change_sprite_color!(last_sprite, 0, 255, 0);
                    });
                }
            }

            // Check non-blockingly if the networking thread has fetched sprite data
            while let Ok(data) = net_to_main_rx.try_recv() {
                println!("[MainThread] Received sprite data! Spawning sprite at ({}, {})...", data.x, data.y);
                let sprite = spawn_sprite!(
                    data.x,
                    data.y,
                    data.width,
                    data.height,
                    data.r,
                    data.g,
                    data.b
                );
                spawned_sprites.push(sprite);
            }

            // Clear screen and render all accumulated sprites
            ffi::clear_screen();
            for &sprite in &spawned_sprites {
                ffi::render_sprite(sprite);
            }
        },
        on_exit: {
            println!("[MainThread] Window closed. Sending Quit command to networking thread...");
            let _ = main_to_net_tx.send(NetworkCommand::Quit);
            if let Some(handle) = networking_thread_handle.take() {
                handle.join().expect("Failed to join networking thread");
            }
            println!("[MainThread] Networking thread joined successfully. Goodbye!");
        }
    }
}
