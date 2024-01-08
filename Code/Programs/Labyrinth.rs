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
    
}
