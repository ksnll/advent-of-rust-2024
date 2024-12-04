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

fn check_safety<T>(line: T) -> Option<Safety>
where
    T: IntoIterator<Item = u32>,
{
    let mut safety: Option<Safety> = None;
    let mut direction = None;
    for (a, b) in line.into_iter().tuple_windows::<(u32, u32)>() {
        let diff = b as i32 - a as i32;
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
    safety
}

async fn process_data(content: &str) -> usize {
    let mut safeties = vec![];
    for line in content.lines() {
        let levels: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let mut safety = check_safety(levels.clone());
        if safety == Some(Safety::Unsafe) {
            for i in 0..levels.len() {
                let mut levels = levels.clone();
                levels.remove(i);
                safety = check_safety(levels);
                if safety == Some(Safety::Safe) {
                    break;
                }
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
        assert_eq!(res, 4);
    }
}
