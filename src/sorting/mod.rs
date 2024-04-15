/// Sorts the elements of the given slice using the insertion sort algorithm.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the slice to be sorted.
///
/// # Examples
///
/// ```
/// use algos::sorting;
///
/// let mut numbers = [4, 2, 3, 1];
/// sorting::insertion_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4]);
/// ```
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();

    for i in 1..len {
        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = key;
    }
}

/// Sorts the elements of the given slice using the merge sort algorithm.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the slice to be sorted.
///
/// # Examples
///
/// ```
/// use algos::sorting;
///
/// let mut numbers = [4, 2, 3, 1];
/// sorting::merge_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4]);
/// ```
pub fn merge_sort<T: Ord + Copy + Default>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    let merged_len = left.len() + right.len();
    let mut merged = vec![Default::default(); merged_len]; // Create a new mutable slice `merged`
    merge(left, right, &mut merged);
    arr.copy_from_slice(&merged);
}

fn merge<T: Ord + Copy>(left: &mut [T], right: &mut [T], merged: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged[k] = left[i];
            i += 1;
        } else {
            merged[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        merged[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        merged[k] = right[j];
        j += 1;
        k += 1;
    }
}
