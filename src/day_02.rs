use std::str::FromStr;

pub const SRC: &'static str = include_str!("day_02.rs");

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

#[derive(Debug, Default, PartialEq)]
struct Game {
    pub id: u32,
    pub outcomes: Vec<Outcome>,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("Game ") {
            if let Some((game_num_str, outecomes_str)) = s.split_once(':') {
                let id = game_num_str.parse().map_err(|_| "Game identifier must be an integer")?;
                let mut outcomes = vec!();
                for outcome_str in outecomes_str.split(';') {
                    let outcome_str = outcome_str.trim();
                    outcomes.push(outcome_str.parse()?);
                }
                Ok(Self { id, outcomes })
            } else {
                Err("':' is expected before game outcomes")
            }
        } else {
            Err("String must start with 'Game ' prefix")
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Outcome {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut outcome = Outcome::default();
        for cube_str in s.split(',') {
            let cube_str = cube_str.trim();
            if let Some((part1, part2)) = cube_str.split_once(' ') {
                let n = part1.parse().map_err(|_| "Cubes count must be an integer")?;
                match part2 {
                    "red" => outcome.red = n,
                    "green" => outcome.green = n,
                    "blue" => outcome.blue =n,
                    _ => {}
                }
            }
        }
        Ok(outcome)
    }
}

pub fn solve_1(input: &str) -> String {
    let mut res = 0;
    'outer: for line in input.split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let game = line.parse::<Game>().unwrap();
        for outcome in game.outcomes {
            if outcome.red > RED_CUBES {
                continue 'outer;
            }
            if outcome.green > GREEN_CUBES {
                continue 'outer;
            }
            if outcome.blue > BLUE_CUBES {
                continue 'outer;
            }
        }
        res += game.id;
    }
    format!("{res}")
}

pub fn solve_2(input: &str) -> String {
    let mut res = 0;
    for line in input.split('\n') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let game = line.parse::<Game>().unwrap();
        let mut min_outcome = Outcome::default();
        for outcome in game.outcomes {
            if outcome.red > min_outcome.red {
                min_outcome.red = outcome.red;
            }
            if outcome.green > min_outcome.green {
                min_outcome.green = outcome.green;
            }
            if outcome.blue > min_outcome.blue {
                min_outcome.blue = outcome.blue;
            }
        }
        res += min_outcome.red * min_outcome.green * min_outcome.blue;
    }
    format!("{res}")
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_parse_outcome() {
        assert_eq!(
            "3 blue, 4 red".parse::<Outcome>().unwrap(),
            Outcome { red: 4, blue: 3, ..Default::default() }
        );
        assert_eq!(
            "2 green".parse::<Outcome>().unwrap(),
            Outcome { green: 2, ..Default::default() }
        );
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse::<Game>().unwrap(),
            Game {
                id: 1,
                outcomes: vec!(
                    Outcome { red: 4, green: 0, blue: 3 },
                    Outcome { red: 1, green: 2, blue: 6 },
                    Outcome { red: 0, green: 2, blue: 0 },
                )
            }
        );
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(
            solve_1(EXAMPLE_INPUT),
            "8".to_string()
        )
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(
            solve_2(EXAMPLE_INPUT),
            "2286".to_string()
        )
    }
}
