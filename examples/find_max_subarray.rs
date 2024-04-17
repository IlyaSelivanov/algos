/// This program demonstrates finding the maximum subarray in a given array.
/// It uses the `find_maximum_subarray` function from the `search_subarray` module.
/// The maximum subarray is defined as the contiguous subarray with the largest sum.
/// The program prints the starting and ending indices of the maximum subarray,
/// along with the sum of its elements.
use algos::searching::search_subarray::find_maximum_subarray;

fn main() {
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let (left, right, sum) = find_maximum_subarray(&arr, 0, arr.len() - 1);
    println!(
        "The maximum subarray is from index {} to {} with a sum of {}",
        left, right, sum
    );
}
