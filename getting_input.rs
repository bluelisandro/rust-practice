// Lisandro Nunez
// 1. Sum from 1 to x
// How to compile: rustc 1.rs
// How to run: ./1

use std::io;

fn main(){
    // Get user input from command line and convert string to integer
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input: i32 = input.trim().parse().unwrap();

    // Call sigma function and output sum
    println!("Sum from 1 to {} is {}", input, sigma(input));
}

// Gets sum from 1 to x recursively
fn sigma(x: i32) -> i32{
    // Recursive base case: If x = 1, return 1
    if x == 1 {
        return 1;
    }

    // Recursive case: If x /= 1, sum is x + sigma(x - 1)
      // So it gets the sum of every number before it from 1 to x-1
    else {
        let sum: i32 = x + sigma(x-1);
        return sum;
    }
}