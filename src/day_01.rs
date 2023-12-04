pub const SRC: &'static str = include_str!("day_01.rs");

pub fn solve_1(input: &str) -> String {
    let mut res = 0;
    for row in input.lines() {
        let mut calibration_value = None;
        for c in row.chars() {
            if c < '1' || c > '9' {
                continue;
            }
            let n = (c as u32 - '0' as u32);
            calibration_value = match calibration_value {
                None => Some((n, n)),
                Some((n1, n2)) => Some((n1, n)),
            }
        }
        if let Some((n1, n2)) = calibration_value {
            res += n1 * 10 + n2;
        }
    }
    return res.to_string();
}

pub fn solve_2(input: &str) -> String {
    let mut res = 0;
    for row in input.lines() {
        let mut calibration_value = None;
        for (i, c) in row.chars().enumerate() {
            let n = if c >= '1' && c <= '9' {
                (c as u32 - '0' as u32)
            } else if row[i..].starts_with("one") {
                1
            } else if row[i..].starts_with("two") {
                2
            } else if row[i..].starts_with("three") {
                3
            } else if row[i..].starts_with("four") {
                4
            } else if row[i..].starts_with("five") {
                5
            } else if row[i..].starts_with("six") {
                6
            } else if row[i..].starts_with("seven") {
                7
            } else if row[i..].starts_with("eight") {
                8
            } else if row[i..].starts_with("nine") {
                9
            } else {
                continue;
            };
            calibration_value = match calibration_value {
                None => Some((n, n)),
                Some((n1, n2)) => Some((n1, n)),
            }
        }
        if let Some((n1, n2)) = calibration_value {
            res += n1 * 10 + n2;
        }
    }
    return res.to_string();
}
