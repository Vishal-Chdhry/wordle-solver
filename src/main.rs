const GAMES: &str = include_str!("../answers.txt");
fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = wordle_solver::algorithms::Naive::new();
        wordle_solver::play(answer, guesser);
    }
}
