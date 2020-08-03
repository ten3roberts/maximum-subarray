#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let sets = [
            vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
            vec![1, 2, 3, 4, 6, 4, 3, -3],
            vec![-5, -3, -7, -4, -4, -5],
            vec![-5, -4, -8, -1, 3, -0],
            vec![3, 4, 5, -3, 6, 100, -4],
            vec![3, -100, 4, 5, 7],
        ];
        let results = [
            vec![4, -1, 2, 1],
            vec![1, 2, 3, 4, 6, 4, 3],
            vec![-3],
            vec![3],
            vec![3, 4, 5, -3, 6, 100],
            vec![4, 5, 7],
        ];

        for (set, result) in sets.iter().zip(results.iter()) {
            println!("Set: {:?}, result: {:?}", set, result);
            let (mut sub, sum) = maximum_subarray::find(set);

            assert_eq!(result, &mut sub);
            assert_eq!(sum, result.iter().sum());

            println!("");
        }
    }
}
