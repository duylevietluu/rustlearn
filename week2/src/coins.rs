use std::io::{self};

// const MAX_LENGTH: usize = 255;
// const MAX_COINS: usize = 100;
const MAX_CHANGE: usize = 100;

const DEBUG: bool = false; // Change DEBUG to true to print extra information

fn calculate_change(
    change: usize, 
    num_coins: usize, 
    coins: &[u32], 
    coins_returned: &mut [isize], 
    next_coin: &mut [usize]
) -> usize {
    let mut min_change = MAX_CHANGE;
    let mut change_found = 0;

    if coins_returned[change] != -1 {
        if DEBUG {
            println!("Found prior result of {} at {}", coins_returned[change], change);
        }
        return coins_returned[change] as usize;
    }

    for i in 0..num_coins {
        let coin_i = coins[i] as usize;
        if change >= coin_i {
            change_found = 1 + calculate_change(change - coin_i, num_coins, coins, coins_returned, next_coin) as isize;
            if change_found < min_change as isize {
                min_change = change_found as usize;

                next_coin[change] = change - coin_i;
                coins_returned[change] = min_change as isize;
            }
        }
    }

    min_change
}

fn main() {
    let mut input = String::new();

    while let Ok(read) = io::stdin().read_line(&mut input) {
        if read == 0 {
            break; // End of input
        }

        // parse line into numbers
        let numbers: Vec<u32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        
        let change: usize = numbers[0] as usize;
        let num_coins: usize = numbers[1] as usize;
        let coins = &numbers[2..];

        println!("{} {}",change,num_coins);

        print!("Using coins ");
        for coin in coins {
            print!("{} ", *coin);
        }

        let mut coins_returned = vec![-1; MAX_CHANGE];
        let mut next_coin = vec![0; MAX_CHANGE];
        coins_returned[0] = 0;
        next_coin[0] = 0;
        // TODO: Initialize coins_returned and next_coin arrays.
        // Don't forget what to do with known coins!
        for coins in coins {
            coins_returned[*coins as usize] = 1;
            next_coin[*coins as usize] = 0;
        }

        let change_count = calculate_change(change, num_coins, coins, &mut coins_returned, &mut next_coin);

        print!(", change for {} required {} coins: ", change, change_count);

        let mut next = change;
        for _ in 0..change_count {
            print!("{} ", next - next_coin[next]);
            next = next_coin[next];
        }
        println!();

        if DEBUG {
            for i in 1..MAX_CHANGE {
                if coins_returned[i] != -1 {
                    println!("{}: {}, {}", i, coins_returned[i], next_coin[i]);
                }
            }
        }

        input.clear();
    }
}
