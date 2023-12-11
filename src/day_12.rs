pub fn solve_1(input: &str) -> crate::PuzzleResult {
    todo!()
}

pub fn solve_2(input: &str) -> crate::PuzzleResult {
    todo!()
}

fn parse(input: &str) -> anyhow::Result<()> {
    let mut lines = input.lines();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
    }

    todo!()
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
    "};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(
            solve_1(EXAMPLE_INPUT)?,
            "".to_string()
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
            "".to_string()
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
