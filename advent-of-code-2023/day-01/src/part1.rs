use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .inspect(|line| {
            dbg!(line);
        })
        .into_iter()
        .map(|line| {
            let mut num = line.chars().filter_map(|c| c.to_digit(10));
            let first = num.next().unwrap();
            let last = if let Some(n) = num.last() { n } else { first };
            let line = first * 10 + last;
            line
        })
        .sum::<u32>();
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
