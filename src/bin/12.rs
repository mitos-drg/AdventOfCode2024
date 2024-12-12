use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
AAAA
BBCD
BBCC
EEEC
";
const TEST2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
const TEST3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn get_area_price(
        input: &Vec<Vec<u8>>,
        start: (usize, usize),
        visited: &mut Vec<Vec<bool>>,
    ) -> usize {
        // Area and perimeter variables
        let mut area = 0;
        let mut perimeter = 0;

        // BFS queue
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back(start);

        // BFS loop
        while !queue.is_empty() {
            // Get current node and mark it as visited
            let (x, y) = queue.pop_front().unwrap();
            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;

            // Increase patch area
            area += 1;

            // Check neigbours
            // left
            if x == 0 || input[x - 1][y] != input[x][y] {
                perimeter += 1
            } else if !visited[x - 1][y] {
                queue.push_back((x - 1, y));
            }

            // right
            if x == input.len() - 1 || input[x + 1][y] != input[x][y] {
                perimeter += 1
            } else if !visited[x + 1][y] {
                queue.push_back((x + 1, y));
            }

            // top
            if y == 0 || input[x][y - 1] != input[x][y] {
                perimeter += 1
            } else if !visited[x][y - 1] {
                queue.push_back((x, y - 1));
            }

            // bottom
            if y == input[x].len() - 1 || input[x][y + 1] != input[x][y] {
                perimeter += 1
            } else if !visited[x][y + 1] {
                queue.push_back((x, y + 1));
            }
        }

        // Return price of fencing the area
        area * perimeter
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Get input
        let input: Vec<Vec<u8>> = reader
            .lines()
            .map(|x| x.unwrap().bytes().collect())
            .collect();
        let mut visited = vec![vec![false; input[0].len()]; input.len()];

        // Create the answer
        let mut answer = 0;

        // Search all patches
        for x in 0..input.len() {
            for y in 0..input[x].len() {
                if !visited[x][y] {
                    answer += get_area_price(&input, (x, y), &mut visited)
                }
            }
        }

        Ok(answer)
    }

    // Expected answer for the test input
    assert_eq!(140, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(772, part1(BufReader::new(TEST2.as_bytes()))?);
    assert_eq!(1930, part1(BufReader::new(TEST3.as_bytes()))?);

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
