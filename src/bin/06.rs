use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Read puzzle input
        let input: Vec<Vec<u8>> = reader
            .lines()
            .map(|x| x.unwrap().bytes().collect())
            .collect();

        // Guard positions
        let mut positions: HashSet<(usize, usize)> = HashSet::new();

        // Find starting guard position
        fn get_start(input: &Vec<Vec<u8>>) -> Option<(usize, usize)> {
            for (i, line) in input.iter().enumerate() {
                for (j, c) in line.iter().enumerate() {
                    if *c == b'^' {
                        return Some((i, j));
                    }
                }
            }

            None
        }
        let (mut x, mut y) = get_start(&input).unwrap();
        let (mut dx, mut dy): (i32, i32) = (-1, 0);

        loop {
            if (x == 0 && dx == -1) || (y == 0 && dy == -1) {
                break;
            }

            if (x == input.len() - 1 && dx == 1) || (y == input[x].len() - 1 && dy == 1) {
                break;
            }

            // Move guard one step
            x = (x as i32 + dx) as usize;
            y = (y as i32 + dy) as usize;

            // Check for collisions
            if input[x][y] == b'#' {
                // Revert guard position
                x = (x as i32 - dx) as usize;
                y = (y as i32 - dy) as usize;

                // Rotate guard
                (dx, dy) = (dy, -dx);
            }

            positions.insert((x, y));
        }

        Ok(positions.len())
    }

    // Expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

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
