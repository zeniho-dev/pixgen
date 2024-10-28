use crate::grid::GRID_CELL_SIZE;
use crate::rules::gol::{count_living_neighbors, update_zygote_state};
use std::time::{Duration, Instant};

pub const ZYGOTE_INITIAL_ENERGY: u32 = 100;
// pub const ZYGOTE_ENERGY_DIVISION_THRESHOLD: u32 = 200;
// pub const ZYGOTE_ENERGY_LOSS_PER_SECOND: u32 = 1;
// pub const ZYGOTE_ENERGY_COST_PER_MOVE: u32 = 1;

#[derive(Clone)]
pub struct Zygote {
    pub x: usize,
    pub y: usize,
    pub energy: u32,
    pub is_alive: bool,
    last_update: Instant, // Track the last update time
}

impl Zygote {
    pub fn new(x: usize, y: usize) -> Self {
        Zygote {
            x,
            y,
            energy: ZYGOTE_INITIAL_ENERGY,
            is_alive: false,
            last_update: Instant::now(),
        }
    }

    // pub fn move_zygote(&mut self) {
    //     if self.is_alive && self.energy > ZYGOTE_ENERGY_COST_PER_MOVE {
    //         // Randomly decide whether to move randomly or follow a pattern
    //         let random_move = rand::random::<bool>();

    //         let (dx, dy) = if random_move {
    //             // Random move: choose a random direction
    //             let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    //             directions[rand::random::<usize>() % directions.len()]
    //         } else {
    //             // Patterned move: follow a simple zigzag pattern
    //             if self.x % 2 == 0 {
    //                 (1, 0) // Move right
    //             } else {
    //                 (0, 1) // Move down
    //             }
    //         };

    //         self.x = (self.x as isize + dx) as usize;
    //         self.y = (self.y as isize + dy) as usize;
    //         self.energy -= ZYGOTE_ENERGY_COST_PER_MOVE;
    //     }
    // }

    // pub fn update(&mut self) {
    //     if self.is_alive {
    //         let now = Instant::now();
    //         if now.duration_since(self.last_update) >= Duration::from_secs(1) {
    //             if self.energy > ZYGOTE_ENERGY_LOSS_PER_SECOND {
    //                 self.energy -= ZYGOTE_ENERGY_LOSS_PER_SECOND;
    //             } else {
    //                 self.is_alive = false;
    //             }
    //             self.last_update = now; // Reset the timer
    //         }
    //     }
    //     self.move_zygote();
    // }

    // pub fn divide(&mut self) -> Option<Zygote> {
    //     if self.energy >= ZYGOTE_ENERGY_DIVISION_THRESHOLD {
    //         self.energy /= 2;
    //         Some(Zygote::new(self.x + 1, self.y))
    //     } else {
    //         None
    //     }
    // }
}

// Function to update all zygotes based on their neighbors
pub fn update_zygotes(zygotes: &mut [Zygote], grid_width: usize, grid_height: usize) {
    let cell_width = grid_width / GRID_CELL_SIZE;
    let cell_height = grid_height / GRID_CELL_SIZE;
    let mut new_states = vec![false; zygotes.len()];

    for y in 0..cell_height {
        for x in 0..cell_width {
            let index = y * cell_width + x;
            if index < zygotes.len() {
                let living_neighbors =
                    count_living_neighbors(x, y, zygotes, cell_width, cell_height);
                new_states[index] = update_zygote_state(zygotes[index].is_alive, living_neighbors);
            }
        }
    }

    for (i, zygote) in zygotes.iter_mut().enumerate() {
        zygote.is_alive = new_states[i];
    }
}
