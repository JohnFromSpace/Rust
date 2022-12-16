use std::io::prelude::*;

fn main() {
    let inputs = std::io::stdin();
    for s in inputs.lock().lines() {
        let mut stack: Vec<f64> = vec![];
        let mut error = false;
        for token in s.unwrap().split_whitespace() {
            if let Ok(x) = token.parse() {
                stack.push(x);
            }
            
            else {
                error = stack.len() < 2;
                if error {break;}
                let (y, x) = (stack.pop().unwrap(), stack.pop().unwrap());  
            }
        }
    }
}
