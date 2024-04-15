use algos::sorting::merge_sort;
use rand::Rng;

// This is a simple example of how to use the merge_sort function.
// We generate an array of 1_000_000 random numbers and sort it.
// The sorting is done in-place, so the original array is sorted.
fn main() {
    let mut rng = rand::thread_rng();

    let mut arr = [0; 1_000_000];
    for i in arr.iter_mut() {
        *i = rng.gen_range(0..100);
    }

    println!("Sorting...");

    merge_sort(&mut arr);

    println!("Done.");
}
