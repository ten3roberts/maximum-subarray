// Finds the largest subarray sum of an array
// Returns the slice and the sum as a tuple
pub fn find<'a>(array: &'a [isize]) -> (&'a [isize], isize) {
    let (mut max_i, mut max_j, mut max_sum) = find_internal(array);
    for start in 1..array.len() {
        // Look for a new subarray at the end of the previous
        let (i, j, s) = find_internal(&array[start..]);
        if s >= max_sum {
            max_i = i + start;
            max_j = j + start;
            max_sum = s;
        }
    }
    (&array[max_i..max_j], max_sum)
}

// Returns the start,end and sum of the nearest subarray
fn find_internal(array: &[isize]) -> (usize, usize, isize) {
    // Attempt to find the first positive or the largest negative number to start at
    let i = array.iter().position(|val| *val >= 0).unwrap_or(
        array
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0,
    );

    // Draw the end as far as possible until a negative number
    let j = array
        .iter()
        .skip(i + 1)
        .position(|val| *val <= 0)
        .unwrap_or(array.len() - i - 1)
        + i
        + 1;

    let mut max_sum: isize = array[i..j].iter().sum();
    let mut max_j = j;
    // Expand j to the end in hopes of a larger number while negatives decrease the sum
    for j in j..array.len() {
        let sum = array[i..j].iter().sum();
        if sum > max_sum {
            max_j = j;
            max_sum = sum;
        }
    }
    let j = max_j;
    (i, j, max_sum)
}
