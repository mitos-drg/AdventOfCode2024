use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Constants
        const VEC: [(i32, i32); 8] = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        // Collect puzzle input
        let input: Vec<Vec<u8>> = reader
            .lines()
            .map(|line| line.unwrap().bytes().collect())
            .collect();

        fn is_word(input: &Vec<Vec<u8>>, dir: (i32, i32), start: (usize, usize)) -> bool {
            // Search variables
            const WORD: [u8; 4] = [b'X', b'M', b'A', b'S'];
            let (mut x, mut y) = start;
            let (dx, dy) = dir;

            for ptr in 0..4 {
                if input.len() > x && input[x].len() > y && input[x][y] == WORD[ptr] {
                    x = (x as i32 + dx) as usize;
                    y = (y as i32 + dy) as usize;
                } else {
                    return false;
                }
            }

            // The word was found
            return true;
        }

        // Search for words in input
        let mut answer = 0;
        for (i, line) in input.iter().enumerate() {
            for (j, letter) in line.iter().enumerate() {
                if *letter == b'X' {
                    for vec in VEC {
                        if is_word(&input, vec, (i, j)) {
                            answer += 1;
                        }
                    }
                }
            }
        }

        Ok(answer)
    }

    // Expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // Collect puzzle input
        let input: Vec<Vec<u8>> = reader
            .lines()
            .map(|line| line.unwrap().bytes().collect())
            .collect();

        // Search for patterns in input
        let mut answer = 0;
        for (i, line) in input.iter().enumerate() {
            for (j, letter) in line.iter().enumerate() {
                if (*letter == b'A')
                    && (1 <= i && i < input.len() - 1 && 1 <= j && j < input[i].len() - 1)
                    && ((input[i + 1][j + 1] == b'M' && input[i - 1][j - 1] == b'S')
                        || (input[i + 1][j + 1] == b'S' && input[i - 1][j - 1] == b'M'))
                    && ((input[i - 1][j + 1] == b'M' && input[i + 1][j - 1] == b'S')
                        || (input[i - 1][j + 1] == b'S' && input[i + 1][j - 1] == b'M'))
                {
                    answer += 1;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
