fn main() {
    let sets = [
        vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
        vec![1, 2, 3, 4, 6, 4, 3, -3],
        vec![-5, -3, -7, -4, -4, -5],
        vec![-5, -4, -8, -1, 3, -0],
        vec![3, 4, 5, -3, 6, 100, -4],
    ];
    let results = [
        vec![4, -1, 2, 1],
        vec![1, 2, 3, 4, 6, 4, 3],
        vec![-3],
        vec![3],
        vec![3, 4, 5, 6, 6, 100],
    ];

    for (set, result) in sets.iter().zip(results.iter()) {
        println!("Set: {:?}, result: {:?}", set, result);
        println!("Max subarray: {:?}", maximum_subarray::find(set));
        println!("");
    }
    panic!();
}
