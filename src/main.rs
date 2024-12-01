use aoc2024::LiteralNumbers;
use macros::aoc;
use reqwest::Method;

#[aoc(2023, 1, 1, "/home/dator/aoccookie")]
fn aoc(input: String) -> String {
    let lines = input.lines();

    lines
        .into_iter()
        .map(|line| {
            let mut first = None;
            let mut last = None;
            line.chars()
                .into_iter()
                .filter(|c| c.is_numeric())
                .for_each(|char| {
                    let num = char as u32 - 48;
                    last.replace(num);
                    if first.is_none() {
                        first = Some(num);
                    }
                });
            last.unwrap_or(0) + first.unwrap_or(0) * 10
        })
        .sum::<u32>()
        .to_string()
}

#[aoc(2023, 1, 2, "/home/dator/aoccookie")]
fn aoc_2(input: String) -> String {
    let lines = input.lines();

    lines
        .into_iter()
        .map(|line| {
            let first = LiteralNumbers::first(&line);
            let last = LiteralNumbers::last(&line);
            println!("first : {first:?} last {last:?}");
            let ret = last.unwrap_or(0) + first.unwrap_or(0) * 10;
            ret
        })
        .sum::<usize>()
        .to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = aoc_2023_1_1().await?;
    let _ = aoc_2023_1_2().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_ex1() {
        let ex_inp = String::from(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        let ret = &super::aoc(ex_inp);
        println!("{ret}");
        assert!(ret == "142");
    }
    #[test]
    fn test_ex2() {
        let ex_inp = String::from(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        let ret = &super::aoc_2(ex_inp);
        println!("{ret}");
        assert!(ret == "281");
    }
}
