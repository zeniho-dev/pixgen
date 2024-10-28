use crate::zygote::Zygote;

pub fn update_zygote_state(is_alive: bool, living_neighbors: usize) -> bool {
    if is_alive {
        // A living cell with fewer than 2 or more than 3 living neighbors dies
        living_neighbors == 2 || living_neighbors == 3
    } else {
        // A dead cell with exactly 3 living neighbors becomes alive
        living_neighbors == 3
    }
}

pub fn count_living_neighbors(
    x: usize,
    y: usize,
    zygotes: &[Zygote],
    width: usize,
    height: usize,
) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // Skip the current zygote
            }
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx < width && ny < height {
                let index = ny * width + nx;
                if zygotes[index].is_alive {
                    count += 1;
                }
            }
        }
    }
    count
}
