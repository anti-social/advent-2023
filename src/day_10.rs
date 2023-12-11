use std::collections::HashMap;

pub fn solve_1(input: &str) -> String {
    let (area, start_loc) = parse(input);

    let pipe = calc_pipe_from_start(&area, start_loc);
    (pipe.len() / 2).to_string()
}

pub fn solve_2(input: &str) -> String {
    let (area, start_loc) = parse(input);

    let pipe = calc_pipe_from_start(&area, start_loc);
    let mut tiles_inside = 0;
    for row_ix in 0..area.len() {
        for col_ix in 0..area[0].len() {
            if pipe.contains_key(&(row_ix, col_ix)) {
                continue;
            }

            let mut intersections = 0;
            let mut touch_via = None;
            for test_row_ix in (0..row_ix).rev() {
                 match pipe.get(&(test_row_ix, col_ix)) {
                     Some('-') => {
                         intersections += 1;
                     }
                     Some('J') => {
                         touch_via = Some('J');
                     }
                     Some('L') => {
                         touch_via = Some('L');
                     }
                     Some('F') => {
                         if let Some('J') = touch_via {
                             intersections += 1;
                         }
                         touch_via = None;
                     }
                     Some('7') => {
                         if let Some('L') = touch_via {
                             intersections += 1;
                         }
                         touch_via = None;
                     }
                     _ => {}
                 }

            }
            if intersections % 2 == 1 {
                tiles_inside += 1;
            }
        }
    }
    tiles_inside.to_string()
}

fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut lines = vec!();
    let mut start_loc = (0, 0);
    for (row_ix, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        lines.push(
            line.chars().enumerate()
                .map(|(col_ix, c)| {
                    if c == 'S' {
                        start_loc = (row_ix, col_ix);
                    }
                    c
                })
                .collect()
        );
    }
    (lines, start_loc)
}

fn calc_pipe_from_start(
    area: &Vec<Vec<char>>,
    start_loc: (usize, usize),
) -> HashMap<(usize, usize), char> {
    let rows = area.len();
    let cols = area[0].len();

    let mut neighbour_locs = vec!();
    if start_loc.0 >= 1 {
        let top_loc = (start_loc.0 - 1, start_loc.1);
        match area[top_loc.0][top_loc.1] {
            '|' | 'F' | '7' => {
                neighbour_locs.push(top_loc);
            }
            _ => {}
        }
    }
    if start_loc.0 < rows - 1 {
        let bottom_loc = (start_loc.0 + 1, start_loc.1);
        match area[bottom_loc.0][bottom_loc.1] {
            '|' | 'L' | 'J' => {
                neighbour_locs.push(bottom_loc);
            }
            _ => {}
        }
    }
    if start_loc.1 >= 1 {
        let left_loc = (start_loc.0, start_loc.1 - 1);
        match area[left_loc.0][left_loc.1] {
            '-' | 'F' | 'L' => {
                neighbour_locs.push(left_loc);
            }
            _ => {}
        }
    }
    if start_loc.1 < cols - 1 {
        let right_loc = (start_loc.0, start_loc.1 + 1);
        match area[right_loc.0][right_loc.1] {
            '-' | '7' | 'J' => {
                neighbour_locs.push(right_loc);
            }
            _ => {}
        }
    }

    if neighbour_locs.len() != 2 {
        panic!("No connected pipes to the start point");
    }
    neighbour_locs.sort();
    calc_pipe(area, start_loc, neighbour_locs[0])
}

fn calc_pipe(
    area: &Vec<Vec<char>>,
    start_loc: (usize, usize),
    cur_loc: (usize, usize),
) -> HashMap<(usize, usize), char> {
    let first_loc = cur_loc;
    let first_tile = area[cur_loc.0][cur_loc.1];
    let mut prev_loc = start_loc;
    let mut cur_loc = cur_loc;
    let mut pipe = HashMap::new();
    let (last_loc, last_tile) = loop {
        pipe.insert(cur_loc, area[cur_loc.0][cur_loc.1]);
        let next_loc = calc_next_loc(area, cur_loc, prev_loc);
        prev_loc = cur_loc;
        let prev_tile = area[prev_loc.0][prev_loc.1];
        cur_loc = next_loc;
        if area[cur_loc.0][cur_loc.1] == 'S' {
            break (prev_loc, prev_tile);
        }
    };

    let start_tile = if first_loc.0 == last_loc.0 {
        '-'
    } else if first_loc.1 == last_loc.1 {
        '|'
    } else if first_loc.1 < last_loc.1 {
        match (first_tile, last_tile) {
            ('|' | '7' | 'F', '-' | '7' | 'J') => 'L',
            _ => '7',
        }
    } else if first_loc.1 > last_loc.1 {
        match (first_tile, last_tile) {
            ('-' | 'J' | '7', '|' | 'J' | 'L') => 'F',
            _ => 'J'
        }
    } else {
        panic!("Cannot find tile for start point")
    };
    pipe.insert(start_loc, start_tile);

    pipe
}

fn calc_next_loc(
    area: &Vec<Vec<char>>,
    cur_loc: (usize, usize),
    prev_loc: (usize, usize),
) -> (usize, usize) {
    let cur_tile = area[cur_loc.0][cur_loc.1];
    match cur_tile {
        '|' => {
            if prev_loc.0 < cur_loc.0 {
                (cur_loc.0 + 1, cur_loc.1)
            } else {
                (cur_loc.0 - 1, cur_loc.1)
            }
        }
        '-' => {
            if prev_loc.1 < cur_loc.1 {
                (cur_loc.0, cur_loc.1 + 1)
            } else {
                (cur_loc.0, cur_loc.1 - 1)
            }
        }
        'L' => {
            if prev_loc.0 < cur_loc.0 {
                (cur_loc.0, cur_loc.1 + 1)
            } else {
                (cur_loc.0 - 1, cur_loc.1)
            }
        }
        'F' => {
            if prev_loc.1 > cur_loc.1 {
                (cur_loc.0 + 1, cur_loc.1)
            } else {
                (cur_loc.0, cur_loc.1 + 1)
            }
        }
        '7' => {
            if prev_loc.1 < cur_loc.1 {
                (cur_loc.0 + 1, cur_loc.1)
            } else {
                (cur_loc.0, cur_loc.1 - 1)
            }
        }
        'J' => {
            if prev_loc.1 < cur_loc.1 {
                (cur_loc.0 - 1, cur_loc.1)
            } else {
                (cur_loc.0, cur_loc.1 - 1)
            }
        }
        _ => panic!("Pipe is broken"),
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;
    use super::*;

    const EXAMPLE_INPUT_1: &'static str = indoc!{"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    "};
    const EXAMPLE_INPUT_2: &'static str = indoc!{"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};
    const EXAMPLE_INPUT_3: &'static str = indoc!{"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};
    const EXAMPLE_INPUT_4: &'static str = indoc!{"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    "};
    const EXAMPLE_INPUT_5: &'static str = indoc!{"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn test_solve_1() {
        assert_eq!(
            solve_1(EXAMPLE_INPUT_1),
            "4".to_string()
        );
        assert_eq!(
            solve_1(EXAMPLE_INPUT_2),
            "8".to_string()
        );
    }

    #[test]
    fn solve_1_with_user_input() -> Result<(), anyhow::Error> {
        let day = util::day_from_filename(file!())?;
        let input = if let Some(input) = util::fetch_user_input(day)? {
            input
        } else {
            return Ok(());
        };

        log::warn!("{}", solve_1(&input));
        Ok(())
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(
            solve_2(EXAMPLE_INPUT_1),
            "1".to_string()
        );
        assert_eq!(
            solve_2(EXAMPLE_INPUT_2),
            "1".to_string()
        );
        assert_eq!(
            solve_2(EXAMPLE_INPUT_3),
            "4".to_string()
        );
        assert_eq!(
            solve_2(EXAMPLE_INPUT_4),
            "8".to_string()
        );
        assert_eq!(
            solve_2(EXAMPLE_INPUT_5),
            "10".to_string()
        );
    }

    #[test]
    fn solve_2_with_user_input() -> Result<(), anyhow::Error> {
        let day = util::day_from_filename(file!())?;
        let input = if let Some(input) = util::fetch_user_input(day)? {
            input
        } else {
            return Ok(());
        };

        log::warn!("{}", solve_2(&input));
        Ok(())
    }
}
