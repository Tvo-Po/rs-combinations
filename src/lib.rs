fn combinations_inner(
    source_arr: &[i32],
    k: usize,
    start: usize,
    current_combination: &mut Vec<i32>,
    k_combinations: &mut Vec<Vec<i32>>,
) {
    if current_combination.len() == k {
        k_combinations.push(current_combination.clone());
        return;
    }

    source_arr[start..source_arr.len()]
        .iter()
        .enumerate()
        .for_each(|(i, element)| {
            current_combination.push(*element);
            combinations_inner(
                source_arr,
                k,
                start + i + 1,
                current_combination,
                k_combinations,
            );
            current_combination.pop();
        });
}

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    assert!(arr.len() <= 20);

    let vec_cap: usize =
        (k..=arr.len()).product::<usize>() / (1..=(arr.len().saturating_sub(k))).product::<usize>();
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(vec_cap);
    let mut current: Vec<i32> = Vec::with_capacity(k);

    combinations_inner(arr, k, 0, &mut current, &mut result);

    return result;
}
