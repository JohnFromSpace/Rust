use rand::Rng;
use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Open,
    Start,
    End,
    Player,
}

struct Labyrinth {
    grid: Vec<Vec<Cell>>,
    player_position: (usize, usize),
    end_position: (usize, usize),
}

impl Labyrinth {
    fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();

         // Generate a random labyrinth
        let mut grid = vec![vec![Cell::Wall; size]; size];

        // Place the player at the start
        let start_position = (rng.gen_range(0..size), rng.gen_range(0..size));
        grid[start_position.0][start_position.1] = Cell::Start;

        // Place the end point
        let end_position = (rng.gen_range(0..size), rng.gen_range(0..size));
        grid[end_position.0][end_position.1] = Cell::End;

        // Make sure the start and end points are not the same
        while start_position == end_position {
            let new_end_position = (rng.gen_range(0..size), rng.gen_range(0..size));
            grid[new_end_position.0][new_end_position.1] = Cell::End;
        }

        Labyrinth {
            grid,
            player_position: start_position,
            end_position,
        }
    }

    fn print(&self) {
        
    }
    
}
