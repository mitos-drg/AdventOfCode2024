use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // How many reports are safe
        let mut answer: usize = 0;

        // Iterate over reports
        'reports: for report in reader.lines() {
            let report = report.unwrap();
            let mut levels = report.split_whitespace();

            // First pass
            let first: i32 = levels.next().unwrap().parse().unwrap();
            let mut last: i32 = levels.next().unwrap().parse().unwrap();
            let dif = first - last;
            let order = dif.signum();

            if 3 < dif.abs() || dif.abs() < 1 {
                continue 'reports;
            }

            // Check if report is safe
            for level in levels {
                let level: i32 = level.parse()?;
                let dif = last - level;
                last = level;

                // Check safety conditions
                if order != dif.signum() || 1 > dif.abs() || dif.abs() > 3 {
                    continue 'reports;
                }
            }

            // Increament counter
            answer += 1;
        }

        // Return answer
        Ok(answer)
    }

    // Expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // How many reports are safe
        let mut answer: usize = 0;

        fn is_safe(report: &[i32], ignore: usize) -> bool {
            // Setup variables
            let mut report = report.iter().enumerate();

            // Ignore first level
            if ignore == 0 {
                report.next();
            }

            // Get first level
            let (_i, &first) = report.next().unwrap();

            // Ignore second level
            if ignore == 1 {
                report.next();
            }

            // Get second level
            let (_j, &last) = report.next().unwrap();
            let mut last = last;

            // Calculate properties
            let dif = first - last;
            let ord = dif.signum();
            let dif = dif.abs();

            // First safety condition
            if !(1 <= dif && dif <= 3) {
                return false;
            }

            for (i, &level) in report {
                if i == ignore {
                    continue;
                }

                // Check safety conditions
                let dif = last - level;
                if ord != dif.signum() || !(1 <= dif.abs() && dif.abs() <= 3) {
                    return false;
                }
                last = level;
            }

            // All check passed
            return true;
        }

        assert!(is_safe(&vec![1, 3, 2, 4, 5], 2));
        assert!(is_safe(&vec![1, 3, 6, 7, 9], 5));

        // Iterate over reports
        for report in reader.lines() {
            let report = report.unwrap();
            let report = report.split_whitespace();
            let report: Vec<i32> = report.map(|x| x.parse::<i32>().unwrap()).collect();
            let mut safe = false;

            for i in 0..report.len() + 1 {
                safe = safe || is_safe(&report, i);
            }

            if safe {
                answer += 1;
            }
        }

        // Return answer
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
