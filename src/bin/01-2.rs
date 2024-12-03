use anyhow::Result;
use tokio::fs;

async fn process_data(content: &str) -> usize {
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();
    for line in content.lines() {
        let mut splitted = line.split_whitespace();
        first_list.push(splitted.next().unwrap().parse::<u32>().unwrap());
        second_list.push(splitted.next().unwrap().parse::<u32>().unwrap());
    }

    first_list.sort();
    second_list.sort();
    first_list
        .iter()
        .map(|value| {
            *value as usize
                * second_list
                    .iter()
                    .filter(|second_value| &value == second_value)
                    .count()
        })
        .sum()
}
#[tokio::main]
async fn main() -> Result<()> {
    let content = fs::read_to_string("01.txt").await?;
    let output = process_data(&content).await;
    dbg!(output);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::process_data;

    #[tokio::test]
    async fn find_value() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        let res = process_data(input).await;
        assert_eq!(res, 31);
    }
}
