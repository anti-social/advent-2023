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

    solve(&times, &distances).to_string()
}

pub fn solve_2(input: &str) -> String {
    let mut lines = input.lines();
    let time_line = lines.next().expect("Time line");
    let times = if let Some((_, times_str)) = time_line.split_once(':') {
        parse_nums(&times_str.trim().replace(" ", ""))
    } else {
        panic!("Missing time line")
    };
    let dist_line = lines.next().expect("Distance line");
    let distances = if let Some((_, distances_str)) = dist_line.split_once(':') {
        parse_nums(&distances_str.trim().replace(" ", ""))
    } else {
        panic!("Missing distance line")
    };
    log::debug!("Times: {times:?}");
    log::debug!("Distances: {distances:?}");

    solve(&times, &distances).to_string()
}

fn parse_nums(s: &str) -> Vec<u64> {
    s.split(' ')
        .filter_map(|v| if v.is_empty() { None } else { v.parse().ok() })
        .collect()
}

fn solve(times: &[u64], distances: &[u64]) -> u64 {
    let mut res = 1;
    for (time, record_dist) in times.iter().zip(distances) {
        let mut num_of_winning_outcomes = 0;
        for speedup_time in 1..*time {
            let left_time = time - speedup_time;
            let speed = speedup_time * 1;
            let dist = speed * left_time;
            if dist > *record_dist {
                num_of_winning_outcomes += 1;
            } else if num_of_winning_outcomes > 0 {
                break;
            }
        }
        res *= num_of_winning_outcomes;
    }
    res
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
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
    fn test_solve_2() {
        assert_eq!(
            solve_2(EXAMPLE_INPUT),
            "71503".to_string()
        );
    }
}
