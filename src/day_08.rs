use std::collections::HashMap;

use anyhow::Context;

use num::integer::lcm;

pub fn solve_1(input: &str) -> crate::PuzzleResult {
    let (navigation, network) = parse(input)?;

    let steps = find_num_steps("AAA", navigation, &network, |n| n == "ZZZ");
    Ok(steps.to_string())
}

pub fn solve_2(input: &str) -> crate::PuzzleResult {
    let (navigation, network) = parse(input)?;

    let steps = network.keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| find_num_steps(n, navigation, &network, |n| n.ends_with("Z")))
        .fold(1, lcm);

    Ok(steps.to_string())
}

fn parse(input: &str) -> anyhow::Result<(&str, HashMap<&str, (&str, &str)>)> {
    let mut lines = input.lines();
    let navigation = lines.next().context("Expect navigation line")?;
    let mut network = HashMap::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((node, neighbours)) = line.split_once('=') {
            let node = node.trim();
            if let Some((left_node, right_node)) = neighbours.split_once(',') {
                let left_node = left_node.trim();
                let left_node = left_node.strip_prefix("(").context("Expect '('")?;
                let right_node = right_node.trim();
                let right_node = right_node.strip_suffix(")").context("Expect ')'")?;
                network.insert(node, (left_node, right_node));
            }
        }
    }

    Ok((navigation, network))
}

pub fn find_num_steps(
    node: &str,
    rules: &str,
    network: &HashMap<&str, (&str, &str)>,
    terminate: impl Fn(&str) -> bool,
) -> u64 {
    let mut current_node = node;
    let mut steps = 0;
    for (i, instruction) in rules.chars().cycle().enumerate() {
        let (left_node, right_node) = network[current_node];
        current_node = match instruction {
            'L' => left_node,
            'R' => right_node,
            _ => continue,
        };
        if terminate(current_node) {
            steps = i + 1;
            break;
        }
    }
    steps as u64
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;
    use super::*;

    const EXAMPLE_INPUT_1: &'static str = indoc!{"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
    "};
    const EXAMPLE_INPUT_2: &'static str = indoc!{"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};
    const EXAMPLE_INPUT_3: &'static str = indoc!{"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(
            solve_1(EXAMPLE_INPUT_1)?,
            "2".to_string()
        );
        assert_eq!(
            solve_1(EXAMPLE_INPUT_2)?,
            "6".to_string()
        );
        Ok(())
    }

    #[test]
    fn solve_1_with_user_input() -> Result<(), anyhow::Error> {
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
            solve_2(EXAMPLE_INPUT_3)?,
            "6".to_string()
        );
        Ok(())
    }

    #[test]
    fn solve_2_with_user_input() -> Result<(), anyhow::Error> {
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
