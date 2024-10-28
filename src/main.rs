use minifb::{Key, Window, WindowOptions};
mod grid;
mod mouse;
mod rules;
mod zygote;

use grid::{draw_grid, Grid, GRID_CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, TOPBAR_HEIGHT};
use mouse::handle_mouse_click;
use std::time::{Duration, Instant};
use zygote::{update_zygotes, Zygote};

#[derive(Clone)]
pub struct GameState {
    pub is_running: bool,
    pub alive_count: usize,
    pub grid: Grid,
    pub zygotes: Vec<Zygote>,
    pub speed: u64,
    pub last_update: Instant,
}

impl GameState {
    pub fn init(&mut self) {
        // Initialize zygotes with their grid positions
        self.is_running = false;
        self.zygotes = (0..GRID_HEIGHT / GRID_CELL_SIZE)
            .flat_map(|y| (0..GRID_WIDTH / GRID_CELL_SIZE).map(move |x| Zygote::new(x, y)))
            .collect();
    }
}

fn main() {
    let mut game_state = GameState {
        is_running: false,
        alive_count: 0,
        grid: Grid {
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
        },
        zygotes: vec![],
        speed: 10,
        last_update: Instant::now(),
    };

    let mut buffer: Vec<u32> = vec![0; GRID_WIDTH * (GRID_HEIGHT + TOPBAR_HEIGHT)];
    let mut window = Window::new(
        "Pixgen - ESC to exit",
        GRID_WIDTH,
        GRID_HEIGHT + TOPBAR_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    // Initialize zygotes with their grid positions
    game_state.init();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Count live and dead zygotes
        game_state.alive_count = game_state.zygotes.iter().filter(|z| z.is_alive).count();

        // Handle mouse input
        handle_mouse_click(&window, &mut game_state.zygotes);

        // Draw the grid
        draw_grid(&mut buffer, &game_state);

        // Toggle play/pause with space key
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            game_state.is_running = !game_state.is_running;
        }

        // Speed down
        if window.is_key_pressed(Key::Down, minifb::KeyRepeat::No) {
            game_state.speed += 10;
        }

        // Speed up
        if window.is_key_pressed(Key::Up, minifb::KeyRepeat::No) {
            if game_state.speed >= 10 {
                game_state.speed -= 10;
            }
        }

        // Reset with 'R' key
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            // Initialize zygotes with their grid positions
            game_state.init();
            game_state.is_running = false;
        }

        if game_state.is_running {
            let now = Instant::now();
            if now.duration_since(game_state.last_update) >= Duration::from_millis(game_state.speed)
            {
                update_zygotes(&mut game_state.zygotes, GRID_WIDTH, GRID_HEIGHT);
                game_state.last_update = Instant::now();
            }
        }

        // Update the window with the buffer
        window
            .update_with_buffer(&buffer, GRID_WIDTH, GRID_HEIGHT + TOPBAR_HEIGHT)
            .unwrap();

        // Add a delay to slow down the game
        std::thread::sleep(Duration::from_millis(10)); // Adjust the duration as needed
    }
}
