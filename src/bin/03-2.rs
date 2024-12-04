use anyhow::Result;
use nom::{bytes::complete::tag, character::complete::digit1, sequence::separated_pair, IResult};
use tokio::fs;

fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, _) = tag("mul(")(input)?;
    let (input, (a, b)) = separated_pair(digit1, tag(","), digit1)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (a.parse().unwrap(), b.parse().unwrap())))
}

fn parse_do(input: &str) -> IResult<&str, &str> {
    tag("do()")(input)
}

fn parse_dont(input: &str) -> IResult<&str, &str> {
    tag("don't()")(input)
}

async fn process_data(content: String) -> Result<u32> {
    let mut rest_of_input = content;
    let mut result = 0;
    let mut enabled = true;
    while !rest_of_input.is_empty() {
        if parse_do(&rest_of_input).is_ok() {
            enabled = true;
        }
        if parse_dont(&rest_of_input).is_ok() {
            enabled = false;
        }
        if let Ok((rest, (a, b))) = parse_mul(&rest_of_input) {
            if enabled {
                result += a * b;
            }
            rest_of_input = rest.to_owned();
        } else {
            rest_of_input.remove(0);
        }
    }
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<()> {
    let content = fs::read_to_string("03.txt").await?;
    let res = process_data(content).await?;
    println!("Result is {}", res);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::process_data;

    #[tokio::test]
    async fn find_value() {
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_owned();
        let res = process_data(input).await;
        assert_eq!(res.unwrap(), 48);
    }
}
