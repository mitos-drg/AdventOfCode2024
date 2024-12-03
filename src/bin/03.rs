use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Initialize answer
        let mut answer: usize = 0;

        // Find all valid instructions
        for line in reader.lines() {
            let line = line.unwrap();
            for cap in Regex::new(r"(?<op>mul)\((?<X>\d+),(?<Y>\d+)\)")
                .unwrap()
                .captures_iter(&line)
            {
                answer += cap["X"].parse::<usize>()? * cap["Y"].parse::<usize>()?;
            }
        }

        // Return answer
        Ok(answer)
    }

    // Expected answer for the test input
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // Initialize answer
        let mut answer: usize = 0;
        let mut enabled = true;

        // Find all valid instructions
        for line in reader.lines() {
            let line = line.unwrap();
            for cap in Regex::new(r"(?<op>(mul)|(don't)|(do))\((?<X>\d+)?,?(?<Y>\d+)?\)")
                .unwrap()
                .captures_iter(&line)
            {
                match &cap["op"] {
                    "mul" => {
                        answer += if enabled {
                            cap["X"].parse::<usize>()? * cap["Y"].parse::<usize>()?
                        } else {
                            0
                        }
                    }
                    "don't" => enabled = false,
                    "do" => enabled = true,
                    _ => continue,
                }
            }
        }

        // Return answer
        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
