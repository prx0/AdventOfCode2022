use std::path::Path;
use tokio::{fs, io::AsyncReadExt};

#[derive(Debug)]
enum Error {
    IO(tokio::io::Error),
}

impl From<tokio::io::Error> for Error {
    fn from(err: tokio::io::Error) -> Self {
        Self::IO(err)
    }
}

type Score = u64;

const ROCK_ATTACK_SCORE: Score = 1;
const PAPER_ATTACK_SCORE: Score = 2;
const SCISSORS_ATTACK_SCORE: Score = 3;

const LOST_SCORE: Score = 0;
const DRAW_SCORE: Score = 3;
const WIN_SCORE: Score = 6;

type Action = char;

const ROCK_ATTACK: Action = 'A';
const PAPER_ATTACK: Action = 'B';
const SCISSORS_ATTACK: Action = 'C';

const ROCK_DEFENSE: Action = 'X';
const PAPER_DEFENSE: Action = 'Y';
const SCISSORS_DEFENSE: Action = 'Z';

const NEED_TO_LOSE: Action = 'X';
const NEED_TO_END: Action = 'Y';
const NEED_TO_WIN: Action = 'Z';

#[derive(Debug, Clone)]
struct Round {
    attack: Action,
    defense: Action,
}

fn apply_strategy(round: &Round) -> Score {
    let Round { attack, defense } = round;
    let defense_score: Score = match defense.to_owned() {
        ROCK_DEFENSE => ROCK_ATTACK_SCORE,
        PAPER_DEFENSE => PAPER_ATTACK_SCORE,
        SCISSORS_DEFENSE => SCISSORS_ATTACK_SCORE,
        _ => panic!("Unsupported defense"),
    };
    match attack.to_owned() {
        ROCK_ATTACK => match defense_score {
            ROCK_ATTACK_SCORE => DRAW_SCORE + defense_score,
            PAPER_ATTACK_SCORE => WIN_SCORE + defense_score,
            SCISSORS_ATTACK_SCORE => LOST_SCORE + defense_score,
            _ => panic!("Unsupported defense"),
        },
        PAPER_ATTACK => match defense_score {
            ROCK_ATTACK_SCORE => LOST_SCORE + defense_score,
            PAPER_ATTACK_SCORE => DRAW_SCORE + defense_score,
            SCISSORS_ATTACK_SCORE => WIN_SCORE + defense_score,
            _ => panic!("Unsupported defense"),
        },
        SCISSORS_ATTACK => match defense_score {
            ROCK_ATTACK_SCORE => WIN_SCORE + defense_score,
            PAPER_ATTACK_SCORE => LOST_SCORE + defense_score,
            SCISSORS_ATTACK_SCORE => DRAW_SCORE + defense_score,
            _ => panic!("Unsupported defense"),
        },
        _ => panic!("Unsupported attack"),
    }
}

fn apply_order(round: &Round) -> Score {
    let Round { attack, defense } = round;
    match attack.to_owned() {
        ROCK_ATTACK => match defense.to_owned() {
            NEED_TO_END => DRAW_SCORE + ROCK_ATTACK_SCORE,
            NEED_TO_LOSE => LOST_SCORE + SCISSORS_ATTACK_SCORE,
            NEED_TO_WIN => WIN_SCORE + PAPER_ATTACK_SCORE,
            _ => panic!("Unsupported order"),
        },
        PAPER_ATTACK => match defense.to_owned() {
            NEED_TO_END => DRAW_SCORE + PAPER_ATTACK_SCORE,
            NEED_TO_LOSE => LOST_SCORE + ROCK_ATTACK_SCORE,
            NEED_TO_WIN => WIN_SCORE + SCISSORS_ATTACK_SCORE,
            _ => panic!("Unsupported order"),
        },
        SCISSORS_ATTACK => match defense.to_owned() {
            NEED_TO_END => DRAW_SCORE + SCISSORS_ATTACK_SCORE,
            NEED_TO_LOSE => LOST_SCORE + PAPER_ATTACK_SCORE,
            NEED_TO_WIN => WIN_SCORE + ROCK_ATTACK_SCORE,
            _ => panic!("Unsupported order"),
        },
        _ => panic!("Unsupported attack"),
    }
}

async fn from_file(path: impl AsRef<Path>) -> Result<Vec<String>, Error> {
    let mut file = fs::File::open(path).await?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).await?;
    let lines: Vec<String> = buf.split('\n').map(|item| item.to_string()).collect();
    Ok(lines)
}

async fn part1(inputs: &[String]) -> Vec<(Round, Score)> {
    let games: Vec<(Round, Score)> = inputs
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let attack: Action = chars.get(0).expect("Expect to be attacked").clone();
            let defense: Action = chars.get(2).expect("Expect to defend").clone();
            let round = Round { attack, defense };
            let score = apply_strategy(&round);
            (round, score)
        })
        .collect();

    games
}

async fn part2(inputs: &[String]) -> Vec<(Round, Score)> {
    let games: Vec<(Round, Score)> = inputs
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let attack: Action = chars.get(0).expect("Expect to be attacked").clone();
            let defense: Action = chars.get(2).expect("Expect to defend").clone();
            let round = Round { attack, defense };
            let score = apply_order(&round);
            (round, score)
        })
        .collect();

    games
}

#[tokio::main]
async fn main() {
    let inputs = from_file(Path::new("input.txt")).await.unwrap();

    let games = part1(&inputs).await;
    let total_score = games
        .into_iter()
        .map(|(_, score)| score)
        .reduce(|sum, score| sum + score);

    println!("Total score: {:?}", total_score);

    let games = part2(&inputs).await;
    let total_score = games
        .into_iter()
        .map(|(_, score)| score)
        .reduce(|sum, score| sum + score);

    println!("Total score if we follow each order: {:?}", total_score);
}
