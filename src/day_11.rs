pub fn solve_1(input: &str) -> String {
    let mut galaxies = parse(input);
    expand(&mut galaxies, 2);
    calc_total_dist(&galaxies).to_string()
}

pub fn solve_2(input: &str) -> String {
    solve_with_expansion_rate(input, 1000_000)
}

pub fn solve_with_expansion_rate(input: &str, expansion_rate: usize) -> String {
    let mut galaxies = parse(input);
    expand(&mut galaxies, expansion_rate);
    calc_total_dist(&galaxies).to_string()
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let input = input.trim();
    let mut galaxies = vec!();
    let mut row_ix = 0;
    let mut col_ix = 0;
    for c in input.chars() {
        if c == '\n' {
            row_ix += 1;
            col_ix = 0;
            continue;
        }
        if c == '#' {
            galaxies.push((row_ix, col_ix));
        }

        col_ix += 1;
    }

    galaxies
}

fn expand(
    galaxies: &mut Vec<(usize, usize)>,
    expansion_rate: usize,
) {
    galaxies.sort_by_key(|g| g.0);
    let mut expansion = 0;
    let mut last_row_ix = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.0 - last_row_ix > 1 {
            expansion += (galaxy.0 - last_row_ix - 1) * (expansion_rate - 1);
        }
        last_row_ix = galaxy.0;
        galaxy.0 += expansion;
    }

    galaxies.sort_by_key(|g| g.1);
    expansion = 0;
    let mut last_col_ix = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.1 - last_col_ix > 1 {
            expansion += (galaxy.1 - last_col_ix - 1) * (expansion_rate - 1);
        }
        last_col_ix = galaxy.1;
        galaxy.1 += expansion;
    }
}

fn calc_total_dist(galaxies: &Vec<(usize, usize)>) -> u64 {
    let mut total_dist = 0u64;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for neighbor_galaxy in &galaxies[i..] {
            total_dist += (galaxy.0 as i64 - neighbor_galaxy.0 as i64).abs() as u64;
            total_dist += (galaxy.1 as i64 - neighbor_galaxy.1 as i64).abs() as u64;
        }
    }
    total_dist
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_solve_1() {
        assert_eq!(
            solve_1(EXAMPLE_INPUT),
            "374".to_string()
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
            solve_with_expansion_rate(EXAMPLE_INPUT, 10),
            "1030".to_string()
        );
        assert_eq!(
            solve_with_expansion_rate(EXAMPLE_INPUT, 100),
            "8410".to_string()
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
