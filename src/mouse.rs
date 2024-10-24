use crate::grid::{GRID_CELL_SIZE, GRID_HEIGHT, GRID_WIDTH, TOPBAR_HEIGHT};
use crate::zygote::Zygote;
use minifb::{MouseButton, Window};

pub fn handle_mouse_click(window: &Window, zygotes: &mut Vec<Zygote>) {
    if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        if window.get_mouse_down(MouseButton::Left) {
            let grid_x = (mouse_x as usize) / GRID_CELL_SIZE;
            let grid_y = (mouse_y as usize - TOPBAR_HEIGHT) / GRID_CELL_SIZE;

            if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                // Check if a zygote already exists at this position
                let position_occupied = zygotes.iter().any(|z| z.x == grid_x && z.y == grid_y);

                if !position_occupied {
                    zygotes.push(Zygote::new(grid_x, grid_y));
                }
            }
        }
    }
}
