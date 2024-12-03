use aoc2024::SequenceMatcher;
use macros::aoc;
use reqwest::Method;

#[aoc(2024, 3, 1, "/home/dator/aoccookie")]
fn aoc(input: String) -> String {
    let lines = input.lines();
    let regex = regex::Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();
    let mut sum = 0;
    for line in lines {
        println!("Line {line}");
        sum += regex
            .captures_iter(line)
            .map(|matches| {
                println!("MATCHES {:?}", matches);
                let x = matches.name("x").unwrap().as_str().parse::<u32>().unwrap();
                let y = matches.name("y").unwrap().as_str().parse::<u32>().unwrap();
                x * y
            })
            .sum::<u32>();
    }
    sum.to_string()
}

#[aoc(2024, 3, 2, "/home/dator/aoccookie")]
fn aoc_2(input: String) -> String {
    let lines = input.lines();
    let regex = regex::Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();
    let do_regex = regex::Regex::new(r"do\(\)").unwrap();
    let donnt_regex = regex::Regex::new(r"don't\(\)").unwrap();
    let mut sum = 0;
    let mut should = true;
    for line in lines {
        let mut ptr = 0;
        loop {
            let substr = &line[ptr..(ptr + 7)];

            if !should
                && do_regex
                    .captures(substr)
                    .is_some_and(|capture| capture.get(0).is_some_and(|el| el.start() == 0))
            {
                ptr += 3;
                should = true;
            } else if should
                && donnt_regex
                    .captures(substr)
                    .is_some_and(|capture| capture.get(0).is_some_and(|el| el.start() == 0))
            {
                ptr += 6;
                should = false;
            } else if should && &substr[0..4] == "mul(" {
                let end = (ptr + 12).min(line.len());
                match regex.captures(&line[ptr..end]) {
                    Some(captures) => {
                        let token = captures.get(0).unwrap().start();
                        if token != 0 {
                            continue;
                        }
                        let x = captures.name("x").unwrap().as_str().parse::<u32>().unwrap();
                        let y = captures.name("y").unwrap().as_str().parse::<u32>().unwrap();
                        sum += x * y;
                        ptr += captures.name("y").unwrap().end();
                    }
                    None => {}
                }
            }
            ptr += 1;
            // If we cannot fit the shortest token in to the next iteration we are done.
            if ptr + 7 >= line.len() {
                break;
            }
        }
    }
    println!("SUM {:?}", sum);
    sum.to_string()
}

#[cfg(test)]
mod test {
    fn input() -> String {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()
    }

    #[test]
    fn test_ex1() {
        let output = &super::aoc(input());
        println!("{output}");
        assert!(output == "161");
    }
    #[test]
    fn test_ex2() {
        assert!(
            &super::aoc_2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                    .to_string()
            ) == "48"
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = aoc_2024_3_1().await?;
    let _ = aoc_2024_3_2().await?;
    Ok(())
}
