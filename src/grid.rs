use std::io::{stdout, Write};

pub const GRID_WIDTH: usize = 800; // Total width in pixels
pub const GRID_HEIGHT: usize = 800; // Total height in pixels (excluding top bar)
pub const TOPBAR_HEIGHT: usize = 20;
pub const GRID_CELL_SIZE: usize = 5; // Size of each grid cell in pixels

pub const COLOR_TOPBAR: u32 = 0x202020; // Dark gray for the top bar
pub const COLOR_GRID_LINE: u32 = 0x404040; // Dark gray for grid lines
pub const COLOR_EMPTY_CELL: u32 = 0x000000; // Black for empty cells

pub fn draw_top_bar(buffer: &mut [u32], width: usize, live_count: usize, dead_count: usize) {
    for y in 0..TOPBAR_HEIGHT {
        for x in 0..width {
            let index = y * width + x;
            buffer[index] = COLOR_TOPBAR;
        }
    }
    // Placeholder for text rendering
    let mut lock = stdout().lock();
    writeln!(lock, "Live: {}, Dead: {}", live_count, dead_count).unwrap();
}

pub fn draw_grid(buffer: &mut [u32], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let index = (y + TOPBAR_HEIGHT) * width + x;
            buffer[index] = if (x % GRID_CELL_SIZE == 0) || (y % GRID_CELL_SIZE == 0) {
                COLOR_GRID_LINE
            } else {
                COLOR_EMPTY_CELL
            };
        }
    }
}
