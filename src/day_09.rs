pub fn solve_1(input: &str) -> String {
    let nums = parse(input);
    let res = nums.iter()
        .map(|l| calc_next_num(&l))
        .fold(0, |acc, n| acc + n);
    res.to_string()
}

pub fn solve_2(input: &str) -> String {
    let nums = parse(input);
    let res = nums.iter()
        .map(|l| calc_prev_num(&l))
        .fold(0, |acc, n| acc + n);
    res.to_string()
}

fn calc_next_num(nums: &[i64]) -> i64 {
    let mut diffs = vec!(nums.to_vec());
    loop {
        let diff = diffs.last().unwrap().windows(2)
            .map(|v| v[1] - v[0])
            .collect::<Vec<_>>();
        if diff.iter().all(|n| *n == 0) {
            break;
        }
        diffs.push(diff);
    }

    let mut last_num = 0;
    for diff in diffs.iter().rev() {
        last_num += diff.last().unwrap();
    }
    last_num
}

fn calc_prev_num(nums: &[i64]) -> i64 {
    let mut diffs = vec!(nums.to_vec());
    loop {
        let diff = diffs.last().unwrap().windows(2)
            .map(|v| v[1] - v[0])
            .collect::<Vec<_>>();
        if diff.iter().all(|n| *n == 0) {
            break;
        }
        diffs.push(diff);
    }

    let mut first_num = 0;
    for diff in diffs.iter().rev() {
        first_num = diff.first().unwrap() - first_num;
    }
    first_num
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    let mut nums = vec!();
    let mut lines = input.lines();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut line_nums = vec!();
        for num_str in line.split(' ') {
            let num_str = num_str.trim();
            line_nums.push(num_str.parse().expect("Number"));
        }
        nums.push(line_nums);
    }

    nums
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn test_solve_1() {
        assert_eq!(
            solve_1(EXAMPLE_INPUT),
            "114".to_string()
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
            solve_2(EXAMPLE_INPUT),
            "2".to_string()
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
