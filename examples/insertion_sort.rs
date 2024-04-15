use algos::sorting::insertion_sort;
use rand::Rng;

/// This program demonstrates the use of insertion sort algorithm to sort an array of integers.
/// We generate an array of 100_000 random numbers and sort it.
/// The sorting is done in-place, so the original array is sorted.
fn main() {
    let mut rng = rand::thread_rng();

    let mut arr = [0; 100_000];
    for i in arr.iter_mut() {
        *i = rng.gen_range(0..100);
    }

    println!("Sorting...");

    insertion_sort(&mut arr);

    println!("Done.");
}
