use anyhow::Result;
use nom::{bytes::complete::tag, character::complete::digit1, sequence::separated_pair, IResult};
use tokio::fs;

fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, _) = tag("mul(")(input)?;
    let (input, (a, b)) = separated_pair(digit1, tag(","), digit1)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (a.parse().unwrap(), b.parse().unwrap())))
}

async fn process_data(content: String) -> Result<u32> {
    let mut rest_of_input = content;
    let mut result = 0;
    while !rest_of_input.is_empty() {
        let try_parse = parse_mul(&rest_of_input);
        if let Ok((rest, (a, b))) = try_parse {
            result += a * b;
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
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_owned();
        let res = process_data(input).await;
        assert_eq!(res.unwrap(), 161);
    }
}
