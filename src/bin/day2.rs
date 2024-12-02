use std::future::{pending, Pending};

use macros::aoc;
use reqwest::Method;

#[aoc(2024, 2, 1, "/home/dator/aoccookie")]
fn aoc(input: String) -> String {
    let lines = input.lines();
    lines
        .into_iter()
        .map(|line| {
            let mut sign = None;
            let mut prev = None;
            let mut invalid = false;
            for (idx, char) in line.split(" ").into_iter().enumerate() {
                // we are the first char.
                if prev.is_none() {
                    prev = Some(char.parse::<u32>().unwrap());

                    continue;
                }
                let inner_prev = unsafe { &mut prev.unwrap_unchecked() };
                let num = char.parse::<u32>().unwrap();
                if sign.is_none() {
                    sign = Some(num > *inner_prev);
                }
                if sign.unwrap() != (num > *inner_prev) {
                    invalid = true;
                    break;
                }
                let diff = u32::abs_diff(num, *inner_prev);
                if diff < 1 || diff > 3 {
                    invalid = true;
                    break;
                }

                prev = Some(num);
            }
            println!("line : {line} safe : {}", !invalid);
            !invalid
        })
        .filter(|el| *el)
        .count()
        .to_string()
}

#[aoc(2024, 2, 2, "/home/dator/aoccookie")]
fn aoc_2(input: String) -> String {
    let lines = input.lines();

    let line_is_valid = |line: Vec<u32>| -> bool {
        let mut sign = None;
        let mut prev = None;

        for (_idx, num) in line.clone().into_iter().enumerate() {
            if prev.is_none() {
                prev = Some(num);

                continue;
            }
            // we are the first char.
            let inner_prev = unsafe { &mut prev.unwrap_unchecked() };
            if sign.is_none() {
                sign = Some(num > *inner_prev);
            }
            if sign.unwrap() != (num > *inner_prev) {
                return false;
            }
            let diff = u32::abs_diff(num, *inner_prev);
            if diff < 1 || diff > 3 {
                return false;
            }

            prev = Some(num);
        }
        true
    };
    lines
        .into_iter()
        .map(|line| {
            let fields = line
                .split(" ")
                .into_iter()
                .map(|el| el.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            if line_is_valid(fields.clone()) {
                return true;
            }
            for idx in 0..fields.len() {
                let mut clone = fields.clone();
                clone.remove(idx);
                if line_is_valid(clone) {
                    return true;
                }
            }
            false
        })
        .filter(|el| *el)
        .count()
        .to_string()
}

#[cfg(test)]
mod test {
    fn input() -> String {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string()
    }

    #[test]
    fn test_ex1() {
        let output = &super::aoc(input());
        println!("Task 1 Output : {output}");
        assert!(output == "2");
    }
    #[test]
    fn test_ex2() {
        let output = &super::aoc_2(input());
        println!("Task 2 Output : {output}");
        assert!(output == "4");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = aoc_2024_2_1().await?;
    let _ = aoc_2024_2_2().await?;
    Ok(())
}
