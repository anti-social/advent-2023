use std::{mem::swap, collections::HashMap};

use sha1::{
    Sha1,
    Digest,
    digest::{
        generic_array::GenericArray,
        typenum::U20,
    }
};

pub fn solve_1(input: &str) -> crate::PuzzleResult {
    let mut platform = parse(input)?;
    // dbg!(&platform);

    while shift_north(&mut platform) != 0 {}

    Ok(calc_weight(&platform).to_string())
}

pub fn solve_2(input: &str) -> crate::PuzzleResult {
    let mut platform = parse(input)?;

    let mut seen_states = HashMap::new();
    let mut weights = vec!();
    let mut cur_ix = 0usize;
    let (offset_to_repeat, cycle_len) = loop {
        while shift_north(&mut platform) != 0 {}
        while shift_west(&mut platform) != 0 {}
        while shift_south(&mut platform) != 0 {}
        while shift_east(&mut platform) != 0 {}

        let hash = calc_hash(&platform);
        if let Some(prev_ix) = seen_states.get(&hash) {
            break (prev_ix, cur_ix - prev_ix);
        }
        let weight = calc_weight(&platform);
        weights.push(weight);
        seen_states.insert(hash, cur_ix);
        cur_ix += 1;
    };

    if cycle_len == 0 {
        anyhow::bail!("Invalid cycle length value")
    }

    let weight_ix = (1_000_000_000 - offset_to_repeat) % cycle_len;
    Ok(weights[offset_to_repeat + weight_ix - 1].to_string())
}

fn parse(input: &str) -> anyhow::Result<Vec<Vec<char>>> {
    let lines = input.lines();
    let mut records = vec!();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        records.push(line.chars().collect());
    }

    Ok(records)
}

fn calc_hash(platform: &[Vec<char>]) -> GenericArray<u8, U20> {
    let mut hasher = Sha1::new();
    let mut buf = [0; 1];
    for row in platform {
        for c in row {
            c.encode_utf8(&mut buf);
            hasher.update(&buf);
        }
    }
    hasher.finalize()
}

fn print_platform(platform: &[Vec<char>]) {
    for row in platform {
        println!(
            "{}",
            row.iter().collect::<String>()
        );
    }
}

fn shift_north(platform: &mut [Vec<char>]) -> usize {
    let mut num_shifted = 0;
    for i in 1..platform.len() {
        let (top, bottom) = platform.split_at_mut(i);
        for (src, dst) in bottom[0].iter_mut().zip(top.last_mut().unwrap()) {
            match (&src, &dst) {
                ('O', '.') => {
                    swap(src, dst);
                    num_shifted += 1;
                }
                _ => {}
            }
        }
    }

    num_shifted
}

fn shift_south(platform: &mut [Vec<char>]) -> usize {
    let mut num_shifted = 0;
    for i in (1..platform.len()).rev() {
        let (top, bottom) = platform.split_at_mut(i);
        for (src, dst) in top.last_mut().unwrap().iter_mut().zip(bottom[0].iter_mut()) {
            match (&src, &dst) {
                ('O', '.') => {
                    swap(src, dst);
                    num_shifted += 1;
                }
                _ => {}
            }
        }
    }

    num_shifted
}

fn shift_west(platform: &mut [Vec<char>]) -> usize {
    let mut num_shifted = 0;
    for src_col_ix in 1..platform[0].len() {
        for row_ix in 0..platform.len() {
            let (left, right) = platform[row_ix].split_at_mut(src_col_ix);
            let dst = left.last_mut().unwrap();
            let src = right.get_mut(0).unwrap();
            match (&src, &dst) {
                ('O', '.') => {
                    swap(src, dst);
                    num_shifted += 1;
                }
                _ => {}
            }
        }
    }

    num_shifted
}

fn shift_east(platform: &mut [Vec<char>]) -> usize {
    let mut num_shifted = 0;
    for src_col_ix in 1..platform[0].len() {
        for row_ix in 0..platform.len() {
            let (left, right) = platform[row_ix].split_at_mut(src_col_ix);
            let src = left.last_mut().unwrap();
            let dst = right.get_mut(0).unwrap();
            match (&src, &dst) {
                ('O', '.') => {
                    swap(src, dst);
                    num_shifted += 1;
                }
                _ => {}
            }
        }
    }

    num_shifted
}

fn calc_weight(platform: &[Vec<char>]) -> usize {
    platform.iter().enumerate()
        .map(|(i, row)| {
            let w = row.iter()
                .map(|&c| (c == 'O') as usize)
                .sum::<usize>();
            (platform.len() - i) * w
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(
            solve_1(EXAMPLE_INPUT)?,
            "136".to_string()
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
            "64".to_string()
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
