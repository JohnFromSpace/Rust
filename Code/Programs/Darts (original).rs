use rand::Rng;
use std::collections::{HashMap, BTreeMap};
use std::io;

#[derive(Debug, Clone)]
struct Player {
    name: String,
    proficiency: f64,
    total_score: usize,
    average_score: f64,
}

impl Player {
    fn new(name: &str, proficiency: f64) -> Self {
        Player {
            name: name.to_string(),
            proficiency,
            total_score: 0,
            average_score: 0.0,
        }    
    } 

    fn throw_dart(&self) -> usize {
         // Calculate the score based on proficiency with some randomization
        let base_score = (self.proficiency * 20.0) as usize;
        let randomization = rand::thread_rng().gen_range(-5.0..5.0);
        let final_score = base_score as isize + randomization as isize;    

        if final_score < 0 {
            0
        } else if final_score > 60 {
            60
        } else {
            final_score as usize
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    players: Vec<Player>,
    rounds: usize,
}

impl Game {
    fn new(players: Vec<Player>, rounds: usize) -> Self {
        Game { players, rounds }    
    }    

    fn play_round(&mut self) {
        for _ in 0..self.rounds {
            for player in &mut self.players {
                let score = player.throw_dart();
                println!("{} scored {} points", player.name, score);
                player.total_score += score;
            }
        }    
    }
}
