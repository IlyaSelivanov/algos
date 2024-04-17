/// Finds the maximum subarray that crosses the midpoint of the given array.
///
/// This function takes an array `arr`, the indices `low`, `mid`, and `high` that define the
/// subarray, and returns a tuple containing the indices of the maximum subarray that crosses the
/// midpoint and the sum of its elements.
///
/// # Arguments
///
/// * `arr` - The array to search for the maximum subarray.
/// * `low` - The starting index of the subarray.
/// * `mid` - The midpoint index of the subarray.
/// * `high` - The ending index of the subarray.
///
/// # Returns
///
/// A tuple `(max_left, max_right, sum)` where:
/// * `max_left` - The starting index of the maximum subarray that crosses the midpoint.
/// * `max_right` - The ending index of the maximum subarray that crosses the midpoint.
/// * `sum` - The sum of the elements in the maximum subarray that crosses the midpoint.
fn find_cross_subarray<T: Ord + Copy>(
    arr: &[T],
    low: usize,
    mid: usize,
    high: usize,
) -> (usize, usize, i32)
where
    i32: std::ops::AddAssign<T>,
{
    let mut left_sum = i32::MIN;
    let mut sum: i32 = 0;
    let mut max_left = 0;
    for i in (low..=mid).rev() {
        sum += arr[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    let mut right_sum = i32::MIN;
    let mut sum = 0;
    let mut max_right = 0;
    for (i, item) in arr.iter().enumerate().take(high + 1).skip(mid + 1) {
        sum += *item;
        if sum > right_sum {
            right_sum = sum;
            max_right = i;
        }
    }

    (max_left, max_right, left_sum + right_sum)
}

/// Finds the maximum subarray in the given array.
///
/// This function takes an array `arr`, the indices `low` and `high` that define the subarray, and
/// returns a tuple containing the indices of the maximum subarray and the sum of its elements.
///
/// # Arguments
///
/// * `arr` - The array to search for the maximum subarray.
/// * `low` - The starting index of the subarray.
/// * `high` - The ending index of the subarray.
///
/// # Returns
///
/// A tuple `(left, right, sum)` where:
/// * `left` - The starting index of the maximum subarray.
/// * `right` - The ending index of the maximum subarray.
/// * `sum` - The sum of the elements in the maximum subarray.
///
/// # Examples
///
/// ```
/// use algos::searching::search_subarray::find_maximum_subarray;
///
/// let numbers = [4, -2, 3, -1, 2, 1, -5, 4];
/// let (left, right, sum) = find_maximum_subarray(&numbers, 0, numbers.len() - 1);
/// assert_eq!(left, 0);
/// assert_eq!(right, 5);
/// assert_eq!(sum, 7);
/// ```
///
/// # Note
///     
/// The function uses a divide-and-conquer approach to find the maximum subarray. It recursively
/// divides the array into two halves and finds the maximum subarray in the left half, right half,
/// and the subarray that crosses the midpoint. The maximum of these three subarrays is the maximum
/// subarray of the original array.
///
/// The time complexity of this function is O(n log n), where n is the number of elements in the
/// array.
///
/// # See also
///     
/// * [Introduction to Algorithms (3rd ed.)](https://mitpress.mit.edu/books/introduction-algorithms-third-edition)
/// * [Wikipedia - Maximum subarray problem](https://en.wikipedia.org/wiki/Maximum_subarray_problem)
pub fn find_maximum_subarray<T: Ord + Copy>(
    arr: &[T],
    low: usize,
    high: usize,
) -> (usize, usize, i32)
where
    i32: std::convert::From<T>,
    i32: std::ops::AddAssign<T>,
{
    if low == high {
        return (low, high, arr[low].into());
    }

    let mid = (low + high) / 2;
    let (left_low, left_high, left_sum) = find_maximum_subarray(arr, low, mid);
    let (right_low, right_high, right_sum) = find_maximum_subarray(arr, mid + 1, high);
    let (cross_low, cross_high, cross_sum) = find_cross_subarray(arr, low, mid, high);

    if left_sum >= right_sum && left_sum >= cross_sum {
        (left_low, left_high, left_sum)
    } else if right_sum >= left_sum && right_sum >= cross_sum {
        (right_low, right_high, right_sum)
    } else {
        (cross_low, cross_high, cross_sum)
    }
}
