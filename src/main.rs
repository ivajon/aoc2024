use macros::aoc;
use reqwest::Method;

#[aoc(2023, 2, 1, "/home/dator/aoccookie")]
fn aoc(input: String) -> String {
    println!("Input : {}", input);
    let res: u32 = input
        .lines()
        .into_iter()
        .map(|line| {
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;
            line.chars()
                .into_iter()
                .filter(|char| char.is_numeric())
                .for_each(|el| {
                    if let None = first {
                        first.replace(el.clone() as u32 - 48);
                    } else {
                        last.replace(el.clone() as u32 - 48);
                    }
                });
            match (first, last) {
                (Some(first), Some(last)) => 10 * first + last,
                (Some(first), None) => first * 10 + first,
                (None, None) => 0,
                _ => unreachable!(),
            }
        })
        .sum();
    let res = format!("{res}");
    println!("Res : {:?}", res);
    res
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = aoc_2023_2_1().await?;
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
        assert!(&super::aoc(ex_inp) == "142");
    }
}
