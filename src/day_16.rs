use std::collections::HashSet;

pub fn solve_1(input: &str) -> crate::PuzzleResult {
    let mut grid = parse(input)?;
    // dbg!(&grid);

    let start_beam = Beam {
        loc: Location { row: 0, col: 0 },
        dir: Direction::Right,
    };
    trace_grid(&mut grid, start_beam);

    Ok(grid.count_energized_tiles().to_string())
}

fn trace_grid(grid: &mut Grid, start_beam: Beam) {
    let mut beams = vec!();
    beams.push(start_beam);

    while !beams.is_empty() {
        // log::debug!("{i}");
        // dbg!(beams.len());
        let mut new_beams = vec!();
        beams.retain_mut(|beam| {
            match beam.tick(grid) {
                Tick::Continue => true,
                Tick::Split { new } => {
                    new_beams.push(new);
                    true
                }
                Tick::EndOfLife => {
                    false
                }
            }
        });
        beams.extend(new_beams);
    }
}

pub fn solve_2(input: &str) -> crate::PuzzleResult {
    let initial_grid = parse(input)?;

    let mut start_beams = vec!();
    for row in 0..initial_grid.len() {
        start_beams.push(Beam {
            loc: Location { row, col: 0 },
            dir: Direction::Right,
        });
        start_beams.push(Beam {
            loc: Location { row, col: initial_grid[0].len() - 1 },
            dir: Direction::Left,
        });
    }
    for col in 0..initial_grid[0].len() {
        start_beams.push(Beam {
            loc: Location { row: 0, col },
            dir: Direction::Down,
        });
        start_beams.push(Beam {
            loc: Location { row: initial_grid.len() - 1, col },
            dir: Direction::Up,
        });
    }

    let mut max_energized_tiles = 0;
    for beam in start_beams {
        let mut grid = initial_grid.clone();
        trace_grid(&mut grid, beam);
        let energized_tiles = grid.count_energized_tiles();
        if energized_tiles > max_energized_tiles {
            max_energized_tiles = energized_tiles;
        }
    }

    Ok(max_energized_tiles.to_string())
}

fn parse(input: &str) -> anyhow::Result<Grid> {
    let lines = input.lines();
    let mut grid = vec!();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut row = vec!();
        for c in line.chars() {
            let mirror = match c {
                '/' => Some('/'),
                '\\' => Some('\\'),
                '|' => Some('|'),
                '-' => Some('-'),
                _ => None,
            };
            row.push(Tile { mirror, energized_by: HashSet::new() });
        }
        grid.push(row);
    }

    Ok(grid)
}

#[derive(Clone, Debug)]
struct Tile {
    pub mirror: Option<char>,
    pub energized_by: HashSet<Direction>,
}

impl Tile {
    fn energize(&mut self, dir: Direction) {
        self.energized_by.insert(dir);
    }
}

trait IGrid {
    fn tile(&mut self, loc: &Location) -> Option<&mut Tile>;

    fn energize(&mut self, beam: &Beam);

    fn count_energized_tiles(&self) -> usize;
}

type Grid = Vec<Vec<Tile>>;

impl IGrid for Grid {
    fn tile(&mut self, loc: &Location) -> Option<&mut Tile> {
        self.get_mut(loc.row).and_then(|row| row.get_mut(loc.col))
    }

    fn energize(&mut self, beam: &Beam) {
        self.tile(&beam.loc).unwrap().energize(beam.dir);
    }

    fn count_energized_tiles(&self) -> usize {
        let mut count = 0;
        for row in self {
            for tile in row {
                if !tile.energized_by.is_empty() {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Location {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Beam {
    pub loc: Location,
    pub dir: Direction,
}

impl Beam {
    fn tick(&mut self, grid: &mut Grid) -> Tick {
        use Direction::*;

        // dbg!(&self.loc);
        let tile = grid.tile(&self.loc).unwrap();
        if tile.energized_by.contains(&self.dir) {
            return Tick::EndOfLife;
        }
        tile.energize(self.dir);

        let (new_dir, split_dir) = match (self.dir, tile.mirror) {
            (Right, Some('/')) => {
                (Up, None)
            }
            (Right, Some('\\')) => {
                (Down, None)
            }
            (Left, Some('/')) => {
                (Down, None)
            }
            (Left, Some('\\')) => {
                (Up, None)
            }
            (Right | Left, Some('|')) if self.loc.row == 0 => {
                (Down, None)
            }
            (Right | Left, Some('|')) if self.loc.row == grid.len() - 1 => {
                (Up, None)
            }
            (Right | Left, Some('|')) => {
                (Up, Some(Down))
            }
            (Up, Some('/')) => {
                (Right, None)
            }
            (Up, Some('\\')) => {
                (Left, None)
            }
            (Down, Some('/')) => {
                (Left, None)
            }
            (Down, Some('\\')) => {
                (Right, None)
            }
            (Up | Down, Some('-')) if self.loc.col == 0 => {
                (Right, None)
            }
            (Up | Down, Some('-')) if self.loc.col == grid[0].len() - 1 => {
                (Left, None)
            }
            (Up | Down, Some('-')) => {
                (Left, Some(Right))
            }
            _ => {
                (self.dir, None)
            }
        };

        self.dir = new_dir;

        let tick_res = if let Some(split_dir) = split_dir {
            let mut split_beam = Beam {
                loc: Location { row: self.loc.row, col: self.loc.col },
                dir: split_dir,
            };
            split_beam.move_toward(grid);
            Tick::Split { new: split_beam }
        } else {
            Tick::Continue
        };

        if !self.move_toward(grid) {
            return Tick::EndOfLife;
        }
        tick_res
    }

    fn move_toward(&mut self, grid: &Grid) -> bool {
        use Direction::*;

        match self.dir {
            Up if self.loc.row != 0 => {
                self.loc.row -= 1;
            }
            Down if self.loc.row != grid.len() - 1 => {
                self.loc.row += 1;
            }
            Left if self.loc.col != 0 => {
                self.loc.col -= 1;
            }
            Right if self.loc.col != grid[0].len() - 1 => {
                self.loc.col += 1;
            }
            _ => return false,
        }
        true
    }
}

enum Tick {
    Continue,
    Split {
        new: Beam,
    },
    EndOfLife,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{r#"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "#};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(
            solve_1(EXAMPLE_INPUT)?,
            "46".to_string()
        );
        Ok(())
    }

    #[test]
    fn solve_1_with_user_input() -> anyhow::Result<()> {
        let day = util::day_from_filename(file!())?;
        let input = if let Some(input) = util::fetch_user_input(day)? {
            input
        } else {
            return Ok(());
        };

        log::warn!("{}", solve_1(&input)?);
        Ok(())
    }

    #[test]
    fn test_solve_2() -> anyhow::Result<()> {
        assert_eq!(
            solve_2(EXAMPLE_INPUT)?,
            "51".to_string()
        );
        Ok(())
    }

    #[test]
    fn solve_2_with_user_input() -> anyhow::Result<()> {
        let day = util::day_from_filename(file!())?;
        let input = if let Some(input) = util::fetch_user_input(day)? {
            input
        } else {
            return Ok(());
        };

        log::warn!("{}", solve_2(&input)?);
        Ok(())
    }
}
