// Lisandro Nunez

// 2. Reverse list of numbers

fn main() {
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Create array
    let mut rev_array: [i32; 5] = array; // Create reversed array by copying non-reversed array
    println!("Before: {:?}", array); // Print non-reversed array
    reverse_list(&array, &mut rev_array, 0, 5); // Call reverse_list function to reverse array
    println!("After: {:?}", rev_array); // Print reversed array
}

// Reverse a given list of integers
fn reverse_list(array: &[i32], rev_array: &mut [i32], index: usize, size: usize) {
    // Recursive base case: If current index is greater than array bounds, kill recursion
    if index > array.len() - 1 {
        return;
    }
    
    // Recursive case: Replace the number at the current index, moving to the right, 
    // with the number at the end of the array, and move to the left as the array is reversed
    else {
        rev_array[index] = array[size-1];
        reverse_list(array, rev_array, index+1, size-1);
    }
}