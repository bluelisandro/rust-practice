// Lisandro Nunez

// 3. Traverse list
// How to compile: rustc 3.rs
// How to run: ./3

use std::convert::TryInto;

fn main() {
    let list: [i32; 10] = [0, 1 , 2, 3, 4, 5, 6, 7, 8, 9];  // Create list
    traverse_list(&list, 10, 0); // Call traverse_list function 
    println!(); // Print empty line
}

// Traverses list and outputs elements recursively
fn traverse_list(list: &[i32], size: i32, index: usize) {
    // Recursive base case: If the current index is out of bounds, then kill the recursion
    if index == (size).try_into().unwrap() {
        return;
    }
    
    // Recursive case: Print whatever is at the current index, then make a recursive call passing in the next index
    else {
        print!("{} ", list[index]);
        traverse_list(&list, size, index+1);
    }
}