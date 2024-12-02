use std::iter::zip;

use anyhow::Result;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();
    let content = fs::read_to_string("01.txt").await?;
    for line in content.lines() {
        let mut splitted = line.split_whitespace();
        first_list.push(splitted.next().unwrap().parse::<i32>().unwrap());
        second_list.push(splitted.next().unwrap().parse::<i32>().unwrap());
    }

    first_list.sort();
    second_list.sort();
    let zipped = zip(first_list, second_list);
    let sum: i32 = zipped.fold(0, |acc, item| acc + (item.0 - item.1).abs());
    dbg!(sum);
    Ok(())
}
