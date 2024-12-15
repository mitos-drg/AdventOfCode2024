use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11,7
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Quantities of robots in quadrants
        let mut quadrants: [usize; 4] = [0; 4];

        // Map dimensions
        let mut lines = reader.lines().map(|line| line.unwrap());
        let (width, height) = lines
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        // Process all robots
        for line in lines {
            let (position, velocity) = line.split(" ").collect_tuple().unwrap();
            let (mut x, mut y) = position[2..]
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            let (dx, dy) = velocity[2..]
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();

            // Simulate robot movement
            for _ in 0..100 {
                x += dx + width;
                y += dy + height;

                x = x % width;
                y = y % height;
            }

            if x < width / 2 {
                if y < height / 2 {
                    quadrants[0] += 1;
                } else if y > height / 2 {
                    quadrants[2] += 1;
                }
            } else if x > width / 2 {
                if y < height / 2 {
                    quadrants[1] += 1;
                } else if y > height / 2 {
                    quadrants[3] += 1;
                }
            }
        }

        // Return answer
        println!("{:?}", quadrants);
        Ok(quadrants.iter().product())
    }

    // Expected answer for the test input
    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
