use std::io::Read;
use std::str::FromStr;

/// The result of a round of Rock-Paper-Scissors.
#[derive(Debug)]
enum Outcome {
    Draw,
    Lose,
    Win,
}

impl From<&Outcome> for usize {
    /// Convert `Outcome` to a score.
    fn from(outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = &'static str;

    /// Attempt to parse `Outcome` from a string.
    fn from_str(st: &str) -> Result<Self, Self::Err> {
        match st {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("Invalid outcome"),
        }
    }
}

/// A move played in a round of Rock-Paper-Scissors.
#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<&Move> for usize {
    /// Convert `Move` to a score.
    fn from(mv: &Move) -> Self {
        match mv {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl FromStr for Move {
    type Err = &'static str;

    /// Attempt to parse `Move` from a string.
    fn from_str(st: &str) -> Result<Self, Self::Err> {
        match st {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Invalid move"),
        }
    }
}

/// A pair of moves played by the opponent and player's response (in that order).
type MovePair = (Move, Move);

impl From<MovePair> for Outcome {
    fn from(mp: MovePair) -> Self {
        let (opponent, response) = mp;
        match opponent {
            Move::Rock => match response {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Lose,
            },
            Move::Paper => match response {
                Move::Rock => Outcome::Lose,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Win,
            },
            Move::Scissors => match response {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Lose,
                Move::Scissors => Outcome::Draw,
            },
        }
    }
}

/// The move played by the opponent and the outcome that must be achieved (in that order).
type Prophecy = (Move, Outcome);

impl From<Prophecy> for Move {
    fn from(pr: Prophecy) -> Self {
        let (opponent, outcome) = pr;
        match opponent {
            Move::Rock => match outcome {
                Outcome::Draw => Move::Rock,
                Outcome::Win => Move::Paper,
                Outcome::Lose => Move::Scissors,
            },
            Move::Paper => match outcome {
                Outcome::Lose => Move::Rock,
                Outcome::Draw => Move::Paper,
                Outcome::Win => Move::Scissors,
            },
            Move::Scissors => match outcome {
                Outcome::Win => Move::Rock,
                Outcome::Lose => Move::Paper,
                Outcome::Draw => Move::Scissors,
            },
        }
    }
}

/// Read, return data from the input file.
fn read_input() -> anyhow::Result<String> {
    let path = std::path::PathBuf::from("input");
    let mut file = std::fs::File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn part_one(data: &str) -> anyhow::Result<usize> {
    let mut score: usize = 0;

    for line in data.lines() {
        // Determine opponent and response moves.
        let opponent = Move::from_str(&line[0..1]).unwrap();
        let response = Move::from_str(&line[2..3]).unwrap();

        // Append the score derived from the response's move.
        score += usize::from(&response);

        // Determine the outcome of the moves.
        let outcome = Outcome::from((opponent, response));

        // Append the score derived from the outcome.
        score += usize::from(&outcome);
    }

    Ok(score)
}

fn part_two(data: &str) -> anyhow::Result<usize> {
    let mut score: usize = 0;

    for line in data.lines() {
        // Determine opponent's move and the desired outcome.
        let opponent = Move::from_str(&line[0..1]).unwrap(); // don't care this is duplicated work
                                                             // from last part
        let outcome = Outcome::from_str(&line[2..3]).unwrap();

        // Append the score derived from the outcome.
        score += usize::from(&outcome);

        // Determine the response from the outcome.
        let response = Move::from((opponent, outcome));

        // Append the score derived from the move.
        score += usize::from(&response);
    }

    Ok(score)
}

fn main() -> anyhow::Result<()> {
    let data = read_input()?;
    println!("Part 1 - Total Score: {}", part_one(&data)?);
    println!("Part 2 - Total Score: {}", part_two(&data)?);
    Ok(())
}
