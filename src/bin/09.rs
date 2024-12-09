use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";
const TEST_SIMPLE: &str = "\
12345
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Create iterator over disc blocks
        let input: Vec<u8> = reader
            .bytes()
            .map(|x| x.unwrap())
            .filter(|x| x.is_ascii_digit())
            .map(|x| x - b'0')
            .collect();
        let mut answer: usize = 0;

        // Variables
        let mut ptr = 0; // Disc head
        let mut last = input.len() / 2; // ID of last written file on disc
        let mut last_size = input[last * 2] as usize; // Size of file with ID = `last`
        let mut i = 0; // current index in the input array
        let mut end = last * 2; // last index of the input array not processed

        // Iterate over disc blocks
        while i < end {
            // Get size of currently processed disc item
            let size = input[i] as usize;

            // Switch behaviour on file/free space
            if i % 2 == 0 {
                // Get file ID
                let id = i / 2;

                // Append file blocks to checksum
                for _ in 0..size {
                    answer += ptr * id;
                    ptr += 1;
                }
            } else {
                // Append file blocks to checksum
                for _ in 0..size {
                    answer += ptr * last;
                    ptr += 1;
                    last_size -= 1;

                    // Check if last file is finished
                    if last_size == 0 {
                        last -= 1;
                        end -= 2;
                        last_size = input[last * 2] as usize;

                        if end < i {
                            break;
                        }
                    }
                }
            }

            // Advance iteration
            i += 1;
        }

        // Count up last block
        if i == end {
            for _ in 0..last_size {
                answer += ptr * last;
                ptr += 1;
            }
        }

        Ok(answer)
    }

    // Expected answer for the test input
    assert_eq!(60, part1(BufReader::new(TEST_SIMPLE.as_bytes()))?);
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

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
