use crate::{zygote::Zygote, GameState};

pub const GRID_WIDTH: usize = 1000; // Total width in pixels
pub const GRID_HEIGHT: usize = 1000; // Total height in pixels (excluding top bar)
pub const TOPBAR_HEIGHT: usize = 20;
pub const GRID_CELL_SIZE: usize = 10; // Size of each grid cell in pixels

pub const COLOR_TOPBAR_RUNNING: u32 = 0x00FF00; // Green for running state
pub const COLOR_TOPBAR_PAUSE: u32 = 0xFFA500; // Orange for pause state
pub const COLOR_GRID_LINE: u32 = 0x404040; // Dark gray for grid lines
pub const COLOR_EMPTY_CELL: u32 = 0x000000; // Black for empty cells
pub const COLOR_LIVING_ZYGOTE: u32 = 0xFFFFFF; // White for living zygotes

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
}

pub fn draw_top_bar(buffer: &mut [u32], game_state: &GameState) {
    let topbar_color = if game_state.is_running {
        COLOR_TOPBAR_RUNNING
    } else {
        COLOR_TOPBAR_PAUSE
    };

    for y in 0..TOPBAR_HEIGHT {
        for x in 0..GRID_WIDTH {
            let index = y * GRID_WIDTH + x;
            buffer[index] = topbar_color;
        }
    }
}

pub fn draw_zygotes(buffer: &mut [u32], width: usize, zygotes: &[Zygote], topbar_height: usize) {
    for zygote in zygotes {
        if zygote.is_alive {
            for dy in 0..GRID_CELL_SIZE {
                for dx in 0..GRID_CELL_SIZE {
                    let index = ((zygote.y * GRID_CELL_SIZE + dy + topbar_height) * width)
                        + (zygote.x * GRID_CELL_SIZE + dx);
                    if index < buffer.len() {
                        buffer[index] = COLOR_LIVING_ZYGOTE;
                    }
                }
            }
        }
    }
}

pub fn draw_grid(buffer: &mut [u32], game_state: &GameState) {
    // Draw the top bar with live and dead counts
    draw_top_bar(buffer, &game_state);

    let width: usize = game_state.grid.width;
    let height: usize = game_state.grid.height;
    // Draw grid lines
    for y in 0..height {
        for x in 0..width {
            let index = (y + TOPBAR_HEIGHT) * width + x;
            if (x % GRID_CELL_SIZE == 0) || (y % GRID_CELL_SIZE == 0) {
                buffer[index] = COLOR_GRID_LINE;
            } else {
                buffer[index] = COLOR_EMPTY_CELL;
            }
        }
    }
    // Draw zygotes using the draw_zygotes function
    draw_zygotes(buffer, width, &game_state.zygotes, TOPBAR_HEIGHT);
}
