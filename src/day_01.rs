pub fn solve_1(input: &str) -> crate::PuzzleResult {
    let mut res = 0;
    for row in input.lines() {
        let mut calibration_value = None;
        for c in row.chars() {
            if let Some(n) = c.to_digit(10) {
                calibration_value = match calibration_value {
                    None => Some((n, n)),
                    Some((n1, _)) => Some((n1, n)),
                }
            }
        }
        if let Some((n1, n2)) = calibration_value {
            res += n1 * 10 + n2;
        }
    }
    Ok(res.to_string())
}

pub fn solve_2(input: &str) -> crate::PuzzleResult {
    let mut res = 0;
    for row in input.lines() {
        let mut calibration_value = None;
        for (i, c) in row.chars().enumerate() {
            let n = if let Some(n) = c.to_digit(10) {
                n
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
                Some((n1, _)) => Some((n1, n)),
            }
        }
        if let Some((n1, n2)) = calibration_value {
            res += n1 * 10 + n2;
        }
    }
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const EXAMPLE_INPUT_1: &'static str = indoc!{"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};
    const EXAMPLE_INPUT_2: &'static str = indoc!{"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(
            solve_1(EXAMPLE_INPUT_1)?,
            "142"
        );
        Ok(())
    }

    #[test]
    fn test_solve_2() -> anyhow::Result<()> {
        assert_eq!(
            solve_2(EXAMPLE_INPUT_2)?,
            "281"
        );
        Ok(())
    }
}
