pub fn solve_1(input: &str) -> crate::PuzzleResult {
    let mirrors = parse(input)?;
    Ok(solve(&mirrors, 0).to_string())
}

pub fn solve_2(input: &str) -> crate::PuzzleResult {
    let mirrors = parse(input)?;
    Ok(solve(&mirrors, 1).to_string())
}

fn solve(mirrors: &[Mirror], num_smudges: u32) -> u32 {
    // dbg!(&mirrors);

    let mut score = 0;
    for mirror in mirrors.iter() {
        let horizontal_split_ix = find_perfect_reflection(&mirror.rows, num_smudges);
        let vertical_split_ix = find_perfect_reflection(&mirror.cols, num_smudges);

        // assert!(!(horizontal_split_ix != 0 && vertical_split_ix != 0));

        // dbg!(horizontal_split_ix);
        // dbg!(vertical_split_ix);
        score += horizontal_split_ix * 100;
        score += vertical_split_ix;
    }

    score as u32
}

fn find_perfect_reflection(rows: &[u64], num_smudges: u32) -> usize {
    let mut reflection_ix = 0;
    let half_mirror_height = rows.len() / 2;
    for reflection_len in (1..half_mirror_height + 1).rev() {
        // dbg!(reflection_len);

        let top_split_ix = reflection_len;
        let is_top_matched = is_reflection(
            &rows[0..top_split_ix],
            &rows[top_split_ix..top_split_ix * 2],
            num_smudges
        );

        let bottom_split_ix = rows.len() - reflection_len;
        let is_bottom_matched = is_reflection(
            &rows[bottom_split_ix..rows.len()],
            &rows[bottom_split_ix - reflection_len..bottom_split_ix],
            num_smudges
        );

        if is_top_matched {
            reflection_ix = top_split_ix;
        }
        if is_bottom_matched {
            reflection_ix = bottom_split_ix;
        }
    }
    reflection_ix
}

fn is_reflection(
    part1: &[u64],
    part2: &[u64],
    num_smudges: u32,
) -> bool {
    part1.iter().zip(part2.iter().rev())
        .map(|(n1, n2)| n1 ^ n2)
        .map(u64::count_ones)
        .sum::<u32>() == num_smudges
}

#[derive(Debug, Default)]
struct Mirror {
    pub rows: Vec<u64>,
    pub cols: Vec<u64>,
}

fn parse(input: &str) -> anyhow::Result<Vec<Mirror>> {
    let lines = input.lines();
    let mut mirrors = vec!();
    let mut cur_rows = vec!();
    let mut cur_cols = vec!();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            if !cur_rows.is_empty() && !cur_cols.is_empty() {
                mirrors.push(Mirror { rows: cur_rows, cols: cur_cols });
                cur_rows = vec!();
                cur_cols = vec!();
            }
            continue;
        }

        let mut row = 0;
        for (col_ix, c) in line.chars().enumerate() {
            if cur_rows.is_empty() {
                cur_cols.resize_with(line.len(), || 0);
            }
            row <<= 1;
            if let Some(col) = cur_cols.get_mut(col_ix) {
                *col <<= 1;
            }
            if c == '#' {
                row |= 1;
                if let Some(col) = cur_cols.get_mut(col_ix) {
                    *col |= 1;
                }
            }
        }
        cur_rows.push(row);
    }
    if !cur_rows.is_empty() && !cur_cols.is_empty() {
        mirrors.push(Mirror { rows: cur_rows, cols: cur_cols });
    }

    Ok(mirrors)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;
    use super::*;

    const EXAMPLE_INPUT_1: &'static str = indoc!{"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};
    const EXAMPLE_INPUT_2: &'static str = indoc!{"
        ##..##..##.
        ######..###
        .####.##.##
        ..........#
        .####.##.##
        .####....##
        ..##..##..#
    "};
    const EXAMPLE_INPUT_3: &'static str = indoc!{"
        ####...
        ##.#...
        ####...
        .#.#...
        ##.####
        ##.##..
        #####..
        ##.#.##
        .##....
        .##.###
        ##..#..
        #..####
        #...#..
    "};
    const EXAMPLE_INPUT_4: &'static str = indoc!{"
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #..#.#..#
    "};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(
            solve_1(EXAMPLE_INPUT_1)?,
            "405".to_string()
        );
        assert_eq!(
            solve_1(EXAMPLE_INPUT_2)?,
            "3".to_string()
        );
        assert_eq!(
            solve_1(EXAMPLE_INPUT_3)?,
            "6".to_string()
        );
        assert_eq!(
            solve_1(EXAMPLE_INPUT_4)?,
            "200".to_string()
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
            solve_2(EXAMPLE_INPUT_1)?,
            "400".to_string()
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
