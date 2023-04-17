// https://www.codewars.com/kata/5672a98bdbdd995fad00000f/train/rust
// Let's play! You have to return which player won! In case of a draw return Draw!.
//
// Examples(Input1, Input2 --> Output):
//
// "scissors", "paper" --> "Player 1 won!"
// "scissors", "rock" --> "Player 2 won!"
// "paper", "paper" --> "Draw!"

fn rps(p1: &str, p2: &str) -> &'static str {
    match (p1, p2) {
        ("scissors", "paper") => "Player 1 won!",
        ("paper", "scissors") => "Player 2 won!",
        ("scissors", "rock") => "Player 2 won!",
        ("rock", "scissors") => "Player 1 won!",
        ("paper", "rock") => "Player 1 won!",
        ("rock", "paper") => "Player 2 won!",
        _ => "Draw!"
    }
}


#[cfg(test)]
mod tests {
    use crate::eight_kyu::rock_paper_scissors_003::rps;

    #[test]
    fn rps_works() {
        let result = rps("scissors", "rock");
        assert_eq!("Player 2 won!", result);
    }
}
