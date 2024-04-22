use crate::number::Number;

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
fn find_cross_subarray<T>(arr: &[T], low: usize, mid: usize, high: usize) -> (usize, usize, T)
where
    T: Ord + Copy + Number + std::ops::AddAssign + std::ops::Add<Output = T>,
{
    let mut left_sum: T = Number::min();
    let mut sum: T = Number::zero();
    let mut max_left = 0;
    for i in (low..=mid).rev() {
        sum += arr[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    let mut right_sum: T = Number::min();
    let mut sum: T = Number::zero();
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
pub fn find_maximum_subarray<T>(arr: &[T], low: usize, high: usize) -> (usize, usize, T)
where
    T: Ord + Copy + Number + std::ops::AddAssign + std::ops::Add<Output = T>,
{
    if low == high {
        return (low, high, arr[low]);
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

/// Finds the maximum subarray using Kadane's algorithm.
///
/// This function takes an array `arr` and returns a tuple containing the indices of the maximum
/// subarray and the sum of its elements. The maximum subarray is defined as the contiguous
/// subarray with the largest sum.
///
/// # Arguments
///
/// * `arr` - The array to search for the maximum subarray.
///
/// # Returns
///
/// A tuple `(start, end, sum)` where:
/// * `start` - The starting index of the maximum subarray.
/// * `end` - The ending index of the maximum subarray.
/// * `sum` - The sum of the elements in the maximum subarray.
///
/// # Examples
///
/// ```
/// use algos::searching::search_subarray::max_subarray_kad;
///
/// let numbers = [4, -2, 3, -1, 2, 1, -5, 4];
/// let (start, end, sum) = max_subarray_kad(&numbers);
/// assert_eq!(start, 0);
/// assert_eq!(end, 5);
/// assert_eq!(sum, 7);
/// ```
///
/// # Note
///
/// Kadane's algorithm is an efficient algorithm for finding the maximum subarray. It iterates
/// through the array, keeping track of the maximum sum found so far and the current sum. If the
/// current sum becomes negative, it is reset to zero, as a negative sum would only decrease the
/// maximum sum. The maximum sum found is the sum of the elements in the maximum subarray. The time
/// complexity of this algorithm is O(n), where n is the number of elements in the array.
pub fn find_maximum_subarray_kad<T>(arr: &[T]) -> (usize, usize, T)
where
    T: Ord
        + Copy
        + std::ops::AddAssign
        + std::ops::Add<Output = T>
        + Default
        + std::cmp::PartialOrd,
{
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    let mut start = 0;
    let mut end = 0;
    let mut temp_start = 0;

    for (i, &item) in arr.iter().enumerate().skip(1) {
        if item > current_sum + item {
            current_sum = item;
            temp_start = i;
        } else {
            current_sum += item;
        }

        if current_sum > max_sum {
            max_sum = current_sum;
            start = temp_start;
            end = i;
        }
    }

    (start, end, max_sum)
}
