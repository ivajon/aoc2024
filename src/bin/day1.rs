use macros::aoc;
use reqwest::Method;

#[aoc(2024, 1, 1, "/home/dator/aoccookie")]
fn aoc(input: String) -> String {
    let mut numbers = [Vec::default(), Vec::default()];
    input.lines().into_iter().for_each(|line| {
        line.split("   ")
            .into_iter()
            .zip(0..=1)
            .for_each(|(el, idx)| numbers[idx].push(el.parse::<u32>().unwrap()))
    });

    numbers[0].sort();
    numbers[1].sort();
    let ret = format!(
        "{}",
        numbers[0]
            .iter()
            .zip(numbers[1].iter())
            .map(|(a, b)| u32::abs_diff(*a, *b))
            .sum::<u32>()
    );
    println!("{}", ret);
    ret
}

#[aoc(2024, 1, 2, "/home/dator/aoccookie")]
fn aoc_2(input: String) -> String {
    let mut numbers = Vec::default();
    let mut hash_set = std::collections::HashMap::new();
    input.lines().into_iter().for_each(|line| {
        line.split("   ")
            .into_iter()
            .zip(0..=1)
            .for_each(|(el, idx)| {
                let val = el.parse::<u32>().unwrap();
                if idx == 0 {
                    numbers.push(val);
                } else {
                    match hash_set.get_mut(&val) {
                        Some(num) => *num += 1,
                        None => {
                            hash_set.insert(val, 1);
                        }
                    }
                }
            })
    });
    let ret = format!(
        "{}",
        numbers
            .iter()
            .map(|el| hash_set.get(el).unwrap_or(&0) * el)
            .sum::<u32>()
    );
    println!("{}", ret);
    ret
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = aoc_2024_1_1().await?;
    let _ = aoc_2024_1_2().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_ex1() {
        let ex_inp = String::from(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert!(&super::aoc(ex_inp) == "11");
    }
    #[test]
    fn test_ex2() {
        let ex_inp = String::from(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert!(&super::aoc_2(ex_inp) == "31");
    }
}
