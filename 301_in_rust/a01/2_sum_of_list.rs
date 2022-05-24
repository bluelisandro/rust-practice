// Lisandro Nunez

// 2. Sum of list of numbers
// How to compile: rustc 2.rs
// How to run: ./2

fn main() {
    // Create array of numbers
    let numbers: [i32; 3] = [1, 2, 3];

    // Call list_sum function and output sum
    println!("Sum of list = {}", list_sum(&numbers, 0));
}

// Compute the sum of a list of numbers recursively
fn list_sum(list: &[i32], index: usize) -> i32 {
    // Recursive base case: We know that size of the list is 2, so if the index is larger than 2, kill the recursion and return 0
    if index > 2 {
        return 0;
    }

    // Recursive case: the sum of the numbers is the current index + the sum of everything to the left of it (calculated by recursively calling the function)
    else {
        let sum: i32 = list[index] + list_sum(&list, index + 1);
        return sum; 
    }
}