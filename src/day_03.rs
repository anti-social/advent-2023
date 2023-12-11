use std::collections::{HashSet, HashMap};

use crate::PuzzleResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Location {
    pub row: u32,
    pub col: u32,
}

impl Location {
    fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
struct Part {
    pub num: u32,
    pub loc: Location,
    pub len: u32,
}

enum State {
    Idle,
    Num {
        loc: Location,
        num: u32,
    }
}

pub fn solve_1(input: &str) -> crate::PuzzleResult {
    let mut state = State::Idle;
    let mut row = 0;
    let mut col = 0;
    let mut parts = vec!();
    let mut symbols = HashSet::<Location>::new();
    for c in input.chars() {
        match state {
            State::Idle => {
                if let Some(d) = c.to_digit(10) {
                    state = State::Num {
                        loc: Location { row, col },
                        num: d,
                    }
                } else if c != '.' && c != '\n' {
                    symbols.insert(Location { row, col });
                }
            }
            State::Num { loc, ref mut num } => {
                if let Some(d) = c.to_digit(10) {
                    *num = *num * 10 + d;
                } else {
                    parts.push(
                        Part {
                            num: *num,
                            loc: loc,
                            len: col - loc.col,
                        }
                    );
                    if c != '.' && c != '\n' {
                        symbols.insert(Location { row, col });
                    }
                    state = State::Idle;
                }
            }
        }
        col += 1;
        if c == '\n' {
            row += 1;
            col = 0;
            continue;
        }
    }

    let mut parts_sum = 0;
    for part in parts {
        let prev_col = if part.loc.col != 0 {
            part.loc.col - 1
        } else {
            part.loc.col
        };
        // Check row above
        if part.loc.row != 0 {
            let above_row = part.loc.row - 1;
            for check_col in prev_col..=(part.loc.col + part.len) {
                if symbols.contains(&Location { row: above_row, col: check_col }) {
                    parts_sum += part.num;
                    continue;
                }
            }
        }
        // Check row under
        for check_col in prev_col..=(part.loc.col + part.len) {
            if symbols.contains(&Location { row: part.loc.row + 1, col: check_col }) {
                parts_sum += part.num;
                continue;
            }
        }
        // Check previous column
        if part.loc.col != 0 {
            if symbols.contains(&Location { row: part.loc.row,  col: part.loc.col - 1 }) {
                parts_sum += part.num;
                continue;
            }
        }
        // Check next column
        if symbols.contains(&Location { row: part.loc.row,  col: part.loc.col + part.len }) {
            parts_sum += part.num;
            continue;
        }
    }

    Ok(parts_sum.to_string())
}

pub fn solve_2(input: &str) -> PuzzleResult {
    let mut state = State::Idle;
    let mut row = 0;
    let mut col = 0;
    let mut part_locs = HashMap::new();
    let mut gears = vec!();
    for c in input.chars() {
        match state {
            State::Idle => {
                if let Some(d) = c.to_digit(10) {
                    state = State::Num {
                        loc: Location::new(row, col),
                        num: d,
                    }
                } else if c == '*' {
                    gears.push(Location::new(row, col));
                }
            }
            State::Num { loc, ref mut num } => {
                if let Some(d) = c.to_digit(10) {
                    *num = *num * 10 + d;
                } else {
                    for part_col in loc.col..col {
                        part_locs.insert(
                            Location::new(loc.row, part_col),
                            *num,
                        );
                    }
                    if c == '*' {
                        gears.push(Location::new(row, col));
                    }
                    state = State::Idle;
                }
            }
        }
        col += 1;
        if c == '\n' {
            row += 1;
            col = 0;
            continue;
        }
    }

    let mut sum = 0;
    for gear_loc in gears {
        let mut part_nums = HashSet::new();

        let prev_col = if gear_loc.col != 0 {
            gear_loc.col - 1
        } else {
            gear_loc.col
        };
        // Check row above
        if gear_loc.row != 0 {
            let above_row = gear_loc.row - 1;
            for check_col in prev_col..=(gear_loc.col + 1) {
                if let Some(num) = part_locs.get(
                    &Location::new(above_row, check_col)
                ) {
                    part_nums.insert(num);
                }
            }
        }
        // Check row under
        for check_col in prev_col..=(gear_loc.col + 1) {
            if let Some(num) = part_locs.get(
                &Location::new(gear_loc.row + 1, check_col)
            ) {
                part_nums.insert(num);
            }
        }
        // Check previous column
        if gear_loc.col != 0 {
            if let Some(num) = part_locs.get(
                &Location::new(gear_loc.row, gear_loc.col - 1)
            ) {
                part_nums.insert(num);
            }
        }
        // Check next column
        if let Some(num) = part_locs.get(
            &Location::new(gear_loc.row, gear_loc.col + 1)
        ) {
            part_nums.insert(num);
        }

        if part_nums.len() == 2 {
            sum += part_nums.iter().fold(1, |acc, n| acc * *n);
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(
            solve_1(EXAMPLE_INPUT)?,
            "4361".to_string()
        );
        Ok(())
    }

    #[test]
    fn test_solve_2() -> anyhow::Result<()> {
        assert_eq!(
            solve_2(EXAMPLE_INPUT)?,
            "467835".to_string()
        );
        Ok(())
    }
}
