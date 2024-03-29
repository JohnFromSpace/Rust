use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn random() -> Move {
        match rand::thread_rng().gen_range(0..3) {
            0 => Move::Rock,
            1 => Move::Paper,
            _ => Move::Scissors,
        }    
    }

    fn beats(&self, other: Move) -> bool {
        (*self == Move::Rock && other == Move::Scissors)
            || (*self == Move::Paper && other == Move::Rock)
            || (*self == Move::Scissors && other == Move::Paper)    
    }
}

struct Strategy {
    history: Vec<Move>,
}

impl Strategy {
    fn new() -> Strategy {
        Strategy { history: Vec::new() }    
    }

    fn observe(&mut self, opponent_move: Move) {
        self.history.push(opponent_move);    
    }

    fn predict(&self) -> Move {
        // Analyze opponent's historical moves and choose the move that maximizes winning chances
        let mut move_counts = HashMap::new();

        for &opponent_move in &self.history {
            let counter = move_counts.entry(opponent_move).or_insert(0);
            *counter += 1;
        }

        let most_common_move = move_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(move, _)| move)
            .unwrap_or(Move::random());

        // Choose the move that beats the most common move
        match most_common_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

fn main() {
    let mut strategy = Strategy::new();  

    for _ in 0..100 {
        // Simulate the game by making random moves
        let my_move = Move::random();
        let opponent_move = Move::random();

        // Observe the opponent's move
        strategy.observe(opponent_move);

        // Predict the opponent's move and print the result
        let predicted_move = strategy.predict();
        println!(
            "My move: {:?}, Opponent's move: {:?}, Predicted move: {:?}, Win: {}",
            my_move,
            opponent_move,
            predicted_move,
            my_move.beats(predicted_move)
        );
    }
}
