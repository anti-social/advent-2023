use arr_macro::arr;

pub fn solve_1(input: &str) -> crate::PuzzleResult {
    let res = parse_1(input)?.iter()
        .map(|h| calc_hash(h))
        .sum::<usize>();

    Ok(res.to_string())
}

pub fn solve_2(input: &str) -> crate::PuzzleResult {
    let mut boxes = arr![Vec::<Lens>::new(); 256];

    for op in parse_2(input)?.iter() {
        match op {
            Operation::Add { lens } => {
                let hash = calc_hash(lens.label);
                let lenses = boxes.get_mut(hash).unwrap();
                let mut updated = false;
                for cur_lens in lenses.iter_mut() {
                    if cur_lens.label == lens.label {
                        cur_lens.focal_len = lens.focal_len;
                        updated = true;
                        break;
                    }
                }
                if !updated {
                    lenses.push(
                        Lens { label: lens.label, focal_len: lens.focal_len }
                    );
                }
            }
            Operation::Remove { label } => {
                let hash = calc_hash(label);
                let lenses = boxes.get_mut(hash).unwrap();
                if let Some(ix) = lenses.iter().position(|l| &l.label == label) {
                    lenses.remove(ix);
                }
            }
        }
    }
    // dbg!(&boxes);

    let focus_power = boxes.iter().enumerate()
        .map(|(i, b)| {
            let box_focus_power = b.iter().enumerate()
                .map(|(i, l)| (i + 1) * l.focal_len as usize)
                .sum::<usize>();
            (i + 1) * box_focus_power
        })
        .sum::<usize>();

    Ok(focus_power.to_string())
}

fn parse_1(input: &str) -> anyhow::Result<Vec<&str>> {
    Ok(input.trim().split(',').collect())
}

fn parse_2(input: &str) -> anyhow::Result<Vec<Operation<'_>>> {
    input.trim()
         .split(',')
         .filter_map(|s| {
             if let Some((label, focal_len_str)) = s.split_once(|c| c == '=' || c == '-') {
                 let op_res = if focal_len_str.is_empty() {
                     Ok(Operation::Remove { label })
                 } else {
                     match focal_len_str.parse() {
                         Ok(focal_len) => Ok(Operation::Add { lens: Lens { label, focal_len } }),
                         Err(e) => Err(anyhow::anyhow!("Focal length must be an integer number: {e}")),
                     }

                 };
                 Some(op_res)
             } else {
                 None
             }
         })
         .collect()
}

fn calc_hash(h: &str) -> usize {
    h.chars()
        .fold(0, |acc, c| {
            (acc + c as usize) * 17 % 256
        })
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_len: u8,
}

#[derive(Debug)]
enum Operation<'a> {
    Add { lens: Lens<'a> },
    Remove { label: &'a str },
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(
            solve_1(EXAMPLE_INPUT)?,
            "1320".to_string()
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
            "145".to_string()
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
