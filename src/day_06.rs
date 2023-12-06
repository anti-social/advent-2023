use std::cmp::Ordering;

pub fn solve_1(input: &str) -> String {
    let mut lines = input.lines();
    let time_line = lines.next().expect("Time line");
    let times = if let Some((_, times_str)) = time_line.split_once(':') {
        parse_nums(times_str.trim())
    } else {
        panic!("Missing time line")
    };
    let dist_line = lines.next().expect("Distance line");
    let distances = if let Some((_, distances_str)) = dist_line.split_once(':') {
        parse_nums(distances_str.trim())
    } else {
        panic!("Missing distance line")
    };
    log::debug!("Times: {times:?}");
    log::debug!("Distances: {distances:?}");

    let mut res = 1;
    for (time, record_dist) in times.iter().zip(distances) {
        let mut num_of_winning_outcomes = 0;
        for speedup_time in 1..*time {
            let left_time = time - speedup_time;
            let speed = speedup_time * 1;
            let dist = speed * left_time;
            if dist > record_dist {
                num_of_winning_outcomes += 1;
            } else if num_of_winning_outcomes > 0 {
                break;
            }
        }
        res *= num_of_winning_outcomes;
    }
    res.to_string()
}

pub fn solve_2(input: &str) -> String {
    let mut lines = input.lines();
    let time_line = lines.next().expect("Time line");
    let time = if let Some((_, times_str)) = time_line.split_once(':') {
        times_str.trim().replace(" ", "").parse::<u64>().expect("Time")
    } else {
        panic!("Missing time line")
    };
    let dist_line = lines.next().expect("Distance line");
    let dist = if let Some((_, distances_str)) = dist_line.split_once(':') {
        distances_str.trim().replace(" ", "").parse::<u64>().expect("Distance")
    } else {
        panic!("Missing distance line")
    };
    log::debug!("Time: {time:?}");
    log::debug!("Distance: {dist:?}");

    // t - total time
    // x - speedup time
    // d - distance
    // x * 1 - speed after speedup (1 is acceleration)
    // t - x - moving time
    // d = (t - x) * t => x^2 - t*x + d = 0
    if let (Some(x1), Some(x2)) = solve_quadratic(-(time as f64), dist as f64) {
        (x1.ceil() as i64) - (x2.ceil() as i64)
    } else {
        0
    }.to_string()
}

fn parse_nums(s: &str) -> Vec<u64> {
    s.split(' ')
        .filter_map(|v| if v.is_empty() { None } else { v.parse().ok() })
        .collect()
}

/**
 * x^2 + bx + c = 0
 * x = b/2 ± √(b^2/4 - c)
 */
fn solve_quadratic(b: f64, c: f64) -> (Option<f64>, Option<f64>) {
    let d = b * b / 4.0 - c;
    match d.total_cmp(&0.0) {
        Ordering::Less => (None, None),
        Ordering::Equal => (Some(b / 2.0), None),
        Ordering::Greater => {
            let square_root_of_d = d.sqrt();
            let half_of_b = b / 2.0;
            (
                Some(half_of_b + square_root_of_d),
                Some(half_of_b - square_root_of_d)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;

    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_solve_1() {
        assert_eq!(
            solve_1(EXAMPLE_INPUT),
            "288".to_string()
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

        log::debug!("{input}");
        log::warn!("{}", solve_1(&input));
        Ok(())
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(
            solve_2(EXAMPLE_INPUT),
            "71503".to_string()
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

        log::debug!("{input}");
        log::warn!("{}", solve_2(&input));
        Ok(())
    }
}
