use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let button = Regex::new(r"Button [AB]: X[+](\d+), Y[+](\d+)").unwrap();
        let prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

        let mut answer = 0;
        let mut lines = reader.lines();
        while let Some(line) = lines.next() {
            // Button A
            let line = line?;
            let (_, [ax, ay]) = button.captures(&line).unwrap().extract();
            let ax: i32 = ax.parse().unwrap();
            let ay: i32 = ay.parse().unwrap();

            // Button B
            let line = lines.next().unwrap()?;
            let (_, [bx, by]) = button.captures(&line).unwrap().extract();
            let bx: i32 = bx.parse().unwrap();
            let by: i32 = by.parse().unwrap();

            // Prize
            let line = lines.next().unwrap()?;
            let (_, [px, py]) = prize.captures(&line).unwrap().extract();
            let px: i32 = px.parse().unwrap();
            let py: i32 = py.parse().unwrap();

            // Empty line
            lines.next();

            // Compute the solution
            let a = (px * by - py * bx) / (ax * by - ay * bx);
            let b = (py * ax - px * ay) / (ax * by - ay * bx);

            // Check if computed solution is correct
            if ax * a + bx * b == px && ay * a + by * b == py {
                answer += 3 * a as usize + b as usize;
            }
        }
        Ok(answer)
    }

    // Expected answer for the test input
    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let button = Regex::new(r"Button [AB]: X[+](\d+), Y[+](\d+)").unwrap();
        let prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let offset: i64 = 10000000000000;

        let mut answer = 0;
        let mut lines = reader.lines();
        while let Some(line) = lines.next() {
            // Button A
            let line = line?;
            let (_, [ax, ay]) = button.captures(&line).unwrap().extract();
            let ax: i64 = ax.parse().unwrap();
            let ay: i64 = ay.parse().unwrap();

            // Button B
            let line = lines.next().unwrap()?;
            let (_, [bx, by]) = button.captures(&line).unwrap().extract();
            let bx: i64 = bx.parse().unwrap();
            let by: i64 = by.parse().unwrap();

            // Prize
            let line = lines.next().unwrap()?;
            let (_, [px, py]) = prize.captures(&line).unwrap().extract();
            let px: i64 = px.parse::<i64>().unwrap() + offset;
            let py: i64 = py.parse::<i64>().unwrap() + offset;

            // Empty line
            lines.next();

            // Compute the solution
            let a = (px * by - py * bx) / (ax * by - ay * bx);
            let b = (py * ax - px * ay) / (ax * by - ay * bx);

            // Check if computed solution is correct
            if ax * a + bx * b == px && ay * a + by * b == py {
                answer += 3 * a as usize + b as usize;
            }
        }
        Ok(answer)
    }

    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
