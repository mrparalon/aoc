use std::fs;
use std::str::FromStr;

fn main() {
    // Read the contents of the file into a string
    let contents = fs::read_to_string("input.txt").unwrap();

    // Split the string into lines
    let lines = contents.split('\n');

    // Count the number of lines that contain one range that contains another
    let count = lines
        .filter(|line| {
            // Parse the line into a pair of ranges
            let mut ranges = Vec::new();
            for s in line.split(',') {
                // Parse each part of the range as a number
                let mut parts = Vec::new();
                for p in s.split('-') {
                    // Handle the error case gracefully
                    match i32::from_str(p) {
                        Ok(num) => parts.push(num),
                        Err(_) => return false,
                    }
                }

                // Skip the line if it doesn't contain a valid pair of numbers
                if parts.len() != 2 {
                    return false;
                }

                // Add the parsed range to the list
                ranges.push((parts[0], parts[1]));
            }

            // Check if one range is completely contained within the other
            (ranges[0].0 >= ranges[1].0 && ranges[0].1 <= ranges[1].1) ||
            (ranges[1].0 >= ranges[0].0 && ranges[1].1 <= ranges[0].1)
        })
        .count();

    // Print the result
    println!("{} lines found.", count);
}

