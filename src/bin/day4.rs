use macros::aoc;
use reqwest::Method;

#[aoc(2024, 4, 1, "/home/dator/aoccookie")]
fn aoc(input: String) -> String {
    let lines = input.lines();
    let data: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut counter = 0;
    let max_x = data.len();
    for x in 0..(max_x) {
        let max_y = data[x].len();
        for y in 0..(max_y) {
            // Time to check...
            //
            if data[x][y] != 'X' {
                continue;
            }
            println!("Found an X on {x},{y}");

            // Check vertical
            let to_parse = ['M', 'A', 'S'];

            if x > 2 {
                let mut valid = true;

                for (idx, x) in ((x - to_parse.len())..x).rev().enumerate() {
                    //println!(
                    //    "Vertial upwrwards Searching  {} {}",
                    //    data[x][y], to_parse[idx]
                    //);
                    if data[x][y] != to_parse[idx] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    counter += 1;
                    println!("Up on {x},{y}");
                }
            }

            if x < (max_x - 3) {
                let mut valid = true;
                for (idx, x) in ((x + 1)..(x + 4)).enumerate() {
                    println!(
                        "Vertial downwards Searching {x},{y}  {} {}",
                        data[x][y], to_parse[idx]
                    );
                    if data[x][y] != to_parse[idx] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    counter += 1;
                    println!("Down on {x},{y}");
                }
            }

            if y > 2 {
                let mut valid = true;
                for (idx, y) in ((y - to_parse.len())..y).rev().enumerate() {
                    if data[x][y] != to_parse[idx] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    counter += 1;
                    println!("Left on {x},{y}");
                }
            }

            if y < (max_y - 3) {
                let mut valid = true;
                for (idx, y) in ((y + 1)..(y + 4)).enumerate() {
                    if data[x][y] != to_parse[idx] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    counter += 1;
                    println!("Right on {x},{y}");
                }
            }
            if y > 2 && x > 2 {
                let mut valid = true;
                for (idx, (x, y)) in ((x - to_parse.len())..x)
                    .zip((y - to_parse.len())..y)
                    .rev()
                    .enumerate()
                {
                    if data[x][y] != to_parse[idx] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    counter += 1;
                    println!("Diagnoal lower right on {x},{y}");
                }
            }
            if y > 2 && x < (max_x - 3) {
                let mut valid = true;
                for (idx, (x, y)) in ((x + 1)..(x + 4))
                    .zip(((y - to_parse.len())..y).rev())
                    .enumerate()
                {
                    if data[x][y] != to_parse[idx] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    counter += 1;
                    println!("Diagnoal upper right on {x},{y}");
                }
            }
            if y < (max_y - 3) && x < (max_x - 3) {
                let mut valid = true;
                for (idx, (x, y)) in ((x + 1)..(x + 4)).zip((y + 1)..(y + 4)).enumerate() {
                    println!("Diag down right {x},{y}  {} {}", data[x][y], to_parse[idx]);
                    if data[x][y] != to_parse[idx] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    counter += 1;
                    println!("Diagnoal lower left on {x},{y}");
                }
            }
            if y < (max_y - 3) && x > 2 {
                let mut valid = true;
                for (idx, (x, y)) in ((x - to_parse.len())..x)
                    .rev()
                    .zip((y + 1)..(y + 4))
                    .enumerate()
                {
                    if data[x][y] != to_parse[idx] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    counter += 1;
                    println!("Diagnoal down left on {x},{y}");
                }
            }
        }
    }
    counter.to_string()
}

#[aoc(2024, 4, 2, "/home/dator/aoccookie")]
fn aoc_2(input: String) -> String {
    let lines = input.lines();
    let data: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut counter = 0;
    let max_x = data.len();
    for x in 1..(max_x - 1) {
        let max_y = data[x].len();
        for y in 1..(max_y - 1) {
            // Time to check...
            //
            if data[x][y] != 'A' {
                continue;
            }

            let region = [
                [data[x - 1][y - 1], data[x - 1][y + 1]],
                [data[x + 1][y - 1], data[x + 1][y + 1]],
            ];
            println!("Region {region:?}");
            if !((region[0][0] == 'M' && region[1][1] == 'S')
                || (region[0][0] == 'S' && region[1][1] == 'M'))
            {
                println!("region[0][0]{}, region[1][1]{}", region[0][0], region[1][1]);
                println!("Invalid first diagonal");
                continue;
            }

            if !((region[0][1] == 'M' && region[1][0] == 'S')
                || (region[0][1] == 'S' && region[1][0] == 'M'))
            {
                println!("Invalid second diagonal");
                continue;
            }

            counter += 1;
            println!("Found an A on {x},{y}");
        }
    }

    counter.to_string()
}

#[cfg(test)]
mod test {
    fn input() -> String {
        "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"
            .to_string()
    }

    #[test]
    fn test_ex1() {
        let output = &super::aoc(input());
        println!("Output : {output}");
        assert!(output == "18");
    }
    #[test]
    fn test_ex2() {
        let output = &super::aoc_2(
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
                .to_string(),
        );
        println!("Output : {output}");
        assert!(output == "9");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = aoc_2024_4_1().await?;
    let _ = aoc_2024_4_2().await?;
    Ok(())
}
