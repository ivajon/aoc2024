use macros::aoc;
use reqwest::Method;

#[aoc(2024, 2, 1, "/home/dator/aoccookie")]
fn aoc(input: String) -> String {
    let _lines = input.lines();
    todo!()
}

#[aoc(2024, 2, 2, "/home/dator/aoccookie")]
fn aoc_2(input: String) -> String {
    let _lines = input.lines();
    todo!()
}

#[cfg(test)]
mod test {
    fn input() -> String {
        "".to_string()
    }

    #[test]
    fn test_ex1() {
        assert!(&super::aoc(input()) == "");
    }
    #[test]
    fn test_ex2() {
        assert!(&super::aoc_2(input()) == "");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = aoc_2024_2_1().await?;
    let _ = aoc_2024_2_2().await?;
    Ok(())
}
