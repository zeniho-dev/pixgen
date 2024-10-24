use minifb::{Key, Window, WindowOptions};
mod grid;
mod mouse;
mod zygote;

use grid::{draw_grid, draw_top_bar, GRID_HEIGHT, GRID_WIDTH, TOPBAR_HEIGHT};
use mouse::handle_mouse_click;
use zygote::draw_zygotes;

fn main() {
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

    let mut zygotes: Vec<zygote::Zygote> = Vec::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle mouse input
        handle_mouse_click(&window, &mut zygotes);

        // Count live and dead zygotes
        let live_count = zygotes.iter().filter(|z| z.is_alive).count();
        let dead_count = zygotes.len() - live_count;

        // Draw the top bar with live and dead counts
        draw_top_bar(&mut buffer, GRID_WIDTH, live_count, dead_count);

        // Draw the grid
        draw_grid(&mut buffer, GRID_WIDTH, GRID_HEIGHT);

        // Update and draw zygotes
        for zygote in &mut zygotes {
            zygote.update();
        }
        draw_zygotes(&mut buffer, GRID_WIDTH, &zygotes, TOPBAR_HEIGHT);

        // Update the window with the buffer
        window
            .update_with_buffer(&buffer, GRID_WIDTH, GRID_HEIGHT + TOPBAR_HEIGHT)
            .unwrap();
    }
}
