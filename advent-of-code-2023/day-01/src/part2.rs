use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let output =
    input
    .replace("one", "o1e")
    .replace("two", "t2o")
    .replace("three", "t3e")
    .replace("four", "f4r")
    .replace("five", "f5e")
    .replace("six", "s6x")
    .replace("seven", "s7n")
    .replace("eight", "e8t")
    .replace("nine", "n9e")
        .lines()
        .inspect(|line| {
            dbg!(line);
        })
        .into_iter()
        .map(|line| {
            let mut num = line.chars().filter_map(|c| c.to_digit(10));
            let first = num.next().unwrap();
            let last = if let Some(n) = num.last(){
                n
            } else { first};
            let line = first*10+last;
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
