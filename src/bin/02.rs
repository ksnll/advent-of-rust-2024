use anyhow::Result;
use itertools::Itertools;
use tokio::fs;

#[derive(Eq, PartialEq, Debug)]
enum Safety {
    Safe,
    Unsafe,
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

async fn process_data(content: &str) -> usize {
    let mut safeties: Vec<Safety> = vec![];
    for line in content.lines() {
        let mut safety: Option<Safety> = None;
        let mut direction = None;
        for (a, b) in line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .tuple_windows::<(u32, u32)>()
        {
            // dbg!(a, b);
            let diff = b as i32 - a as i32;
            // dbg!(diff.signum(), &direction);
            match (diff.signum(), &direction) {
                (0, _) => {
                    safety = Some(Safety::Unsafe);
                }
                (-1, None) => {
                    safety = Some(Safety::Safe);
                    direction = Some(Direction::Decreasing);
                }
                (1, None) => {
                    safety = Some(Safety::Safe);
                    direction = Some(Direction::Increasing);
                }
                (-1, Some(Direction::Decreasing)) => {
                    safety = Some(Safety::Safe);
                }
                (-1, Some(Direction::Increasing)) => {
                    safety = Some(Safety::Unsafe);
                }
                (1, Some(Direction::Increasing)) => {
                    safety = Some(Safety::Safe);
                }
                (1, Some(Direction::Decreasing)) => {
                    safety = Some(Safety::Unsafe);
                }
                _ => panic!("Unexpected direction"),
            };
            if (b as i32 - a as i32).abs() > 3 {
                safety = Some(Safety::Unsafe)
            }
            if safety == Some(Safety::Unsafe) {
                break;
            }
        }
        safeties.push(safety.unwrap());
    }
    dbg!(&safeties);
    safeties.iter().filter(|x| x == &&Safety::Safe).count()
}

#[tokio::main]
async fn main() -> Result<()> {
    let content = fs::read_to_string("02.txt").await?;
    let output = process_data(&content).await;
    dbg!(output);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::process_data;

    #[tokio::test]
    async fn find_value() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let res = process_data(input).await;
        assert_eq!(res, 2);
    }
}
