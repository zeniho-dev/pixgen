use crate::grid::GRID_CELL_SIZE;
use std::time::{Duration, Instant};

pub const ZYGOTE_INITIAL_ENERGY: u32 = 100;
pub const ZYGOTE_ENERGY_DIVISION_THRESHOLD: u32 = 200;
pub const ZYGOTE_ENERGY_LOSS_PER_SECOND: u32 = 1;
pub const ZYGOTE_ENERGY_COST_PER_MOVE: u32 = 1;

pub const COLOR_LIVING_ZYGOTE: u32 = 0xFFFFFF; // White for living zygotes
pub const COLOR_DEAD_ZYGOTE: u32 = 0x808080; // Gray for dead zygotes

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
            is_alive: true,
            last_update: Instant::now(),
        }
    }

    pub fn move_zygote(&mut self) {
        if self.is_alive && self.energy > ZYGOTE_ENERGY_COST_PER_MOVE {
            // Randomly decide whether to move randomly or follow a pattern
            let random_move = rand::random::<bool>();

            let (dx, dy) = if random_move {
                // Random move: choose a random direction
                let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                directions[rand::random::<usize>() % directions.len()]
            } else {
                // Patterned move: follow a simple zigzag pattern
                if self.x % 2 == 0 {
                    (1, 0) // Move right
                } else {
                    (0, 1) // Move down
                }
            };

            self.x = (self.x as isize + dx) as usize;
            self.y = (self.y as isize + dy) as usize;
            self.energy -= ZYGOTE_ENERGY_COST_PER_MOVE;
        }
    }

    pub fn update(&mut self) {
        if self.is_alive {
            let now = Instant::now();
            if now.duration_since(self.last_update) >= Duration::from_secs(1) {
                if self.energy > ZYGOTE_ENERGY_LOSS_PER_SECOND {
                    self.energy -= ZYGOTE_ENERGY_LOSS_PER_SECOND;
                } else {
                    self.is_alive = false;
                }
                self.last_update = now; // Reset the timer
            }
        }
        self.move_zygote();
    }

    pub fn divide(&mut self) -> Option<Zygote> {
        if self.energy >= ZYGOTE_ENERGY_DIVISION_THRESHOLD {
            self.energy /= 2;
            Some(Zygote::new(self.x + 1, self.y))
        } else {
            None
        }
    }
}

pub fn draw_zygotes(buffer: &mut [u32], width: usize, zygotes: &[Zygote], topbar_height: usize) {
    for zygote in zygotes {
        for dy in 0..GRID_CELL_SIZE {
            for dx in 0..GRID_CELL_SIZE {
                let index = ((zygote.y * GRID_CELL_SIZE + dy + topbar_height) * width)
                    + (zygote.x * GRID_CELL_SIZE + dx);
                buffer[index] = if zygote.is_alive {
                    COLOR_LIVING_ZYGOTE
                } else {
                    COLOR_DEAD_ZYGOTE
                };
            }
        }
    }
}
