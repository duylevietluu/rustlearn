// Part 1 - Frequency Analysis
// Read text from stdin, count character frequencies,
// and print a summary table including most and least frequent characters.

use std::io::{stdin, BufRead};
use std::cmp::{max_by_key,min_by_key};

fn main() {
    // Read input
    let mut arr: [u32; 256] = [0; 256];

    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            for mut c in line.chars() {
                // Process each character c
                if c.is_uppercase() {
                    c = c.to_ascii_lowercase();
                }
                arr[c as usize] += 1;
            }
        } else {
            println!("Read failed");
            return;
        }
    }

    // Find max and min
    let mut maxid: usize = 'a' as usize;
    let mut minid: usize = 'a' as usize;
    let mut sum: u32 = arr[maxid];

    for i in 'b' as usize..'z' as usize {
        maxid = max_by_key(maxid, i, |x| arr[*x]);
        minid = min_by_key(minid, i, |x| arr[*x]);
        sum += arr[i];
    }

    // Print table
    println!("char        Frequencies         Percentage");
    for i in 'a' as u8..'z' as u8 {
        let x = arr[i as usize];
        println!("{}:{:20}{:20.4}", i as char, x, x as f64 / sum as f64 * 100f64);
    }
    println!("Maximum character: {}", maxid as u8 as char);
    println!("Minimum character: {}", minid as u8 as char);

}
