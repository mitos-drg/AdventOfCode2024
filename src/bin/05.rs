use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Collect printing ruleset - every number stores it's 'dependencies'
        let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut lines = reader.lines();
        for line in lines.by_ref() {
            let line = line.unwrap();

            // Detect end of rules
            if line == "" {
                break;
            }

            // Add rules
            let (x, y) = line
                .split("|")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let rule = rules.entry(y).or_insert(Vec::<usize>::new());
            rule.push(x);
        }

        // Check printing orders
        let mut answer = 0;
        'orders: for line in lines {
            let line = line.unwrap();
            let order: Vec<usize> = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let mut dependencies: Vec<usize> = Vec::new();

            for x in order.iter() {
                if dependencies.contains(&x) {
                    continue 'orders;
                }
                if rules.contains_key(&x) {
                    for &d in rules[&x].iter() {
                        dependencies.push(d);
                    }
                }
            }
            answer += order[order.len() / 2];
        }

        // Return answer
        Ok(answer)
    }

    // Expected answer for the test input
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // Collect printing ruleset - every number stores it's 'dependencies'
        let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut lines = reader.lines();
        for line in lines.by_ref() {
            let line = line.unwrap();

            // Detect end of rules
            if line == "" {
                break;
            }

            // Add rules
            let (x, y) = line
                .split("|")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let rule = rules.entry(y).or_insert(Vec::<usize>::new());
            rule.push(x);
        }

        // Check printing orders
        let mut answer = 0;
        for line in lines {
            let line = line.unwrap();
            let mut order: Vec<usize> = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let mut dependencies: Vec<usize> = Vec::new();

            for x in order.iter() {
                if dependencies.contains(&x) {
                    order.sort_by(|x, y| {
                        if rules.contains_key(y) && rules[y].contains(x) {
                            Ordering::Less
                        } else if rules.contains_key(x) && rules[x].contains(y) {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    });
                    answer += order[order.len() / 2];
                    break;
                }
                if rules.contains_key(&x) {
                    for &d in rules[&x].iter() {
                        dependencies.push(d);
                    }
                }
            }
        }

        // Return answer
        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
