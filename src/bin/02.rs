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

    fn is_safe(report: &[i32], ignore: usize) -> bool {
        let mut report = report.iter().enumerate();

        // First pass
        let (i, first) = report.next().unwrap();
        let (j, mut last) = report.next().unwrap();
        let dif = first - last;
        let order = dif.signum();

        if 3 < dif.abs() || dif.abs() < 1 {
            return false;
        }

        // Check if report is safe
        for level in report {
            let dif = last - level;

            // Check safety conditions
            if order != dif.signum() || 1 > dif.abs() || dif.abs() > 3 {
                if ignored {
                    return false;
                } else {
                    ignored = true;
                    continue;
                }
            }

            last = level.clone();
        }

        // All checks has passed
        return true;
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // How many reports are safe
        let mut answer: usize = 0;

        // Iterate over reports
        for report in reader.lines() {
            let report = report.unwrap();
            let report = report.split_whitespace();
            let report: Vec<i32> = report.map(|x| x.parse::<i32>().unwrap()).collect();

            if is_safe(&report, false) {
                // Increment counter
                answer += 1;
            } else if is_safe(&report[1..], true) {
                answer += 1;
            } else {
                let mut modified: Vec<i32> = vec![report[0]];
                modified.append(&mut report[2..].iter().copied().collect());
                if is_safe(&modified, true) {
                    answer += 1;
                } else {
                    println!("{:?}", report);
                }
            }
        }

        // Return answer
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    println!("");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
