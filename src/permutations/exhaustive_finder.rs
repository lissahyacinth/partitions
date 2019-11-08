use std::iter::Iterator;

pub(crate) fn exhaustive_permutations(input: Vec<usize>, processed_input: Vec<bool>) -> Vec<Vec<usize>> {
    if processed_input
        .iter()
        .fold(0_usize, |acc, elem| acc + if *elem { 1 } else { 0 })
        == (input.len() - 1)
    {
        return vec![input];
    } else {
        (0_usize..input.len())
            .zip(processed_input.clone())
            .filter(|(_, processed)| !*processed)
            .map(|(swap_index, _)| {
                let mut swap_processed_input = processed_input.clone();
                let mut swap_input = input.clone();
                let initial_swap_input = swap_processed_input
                    .binary_search(&false)
                    .unwrap_or(0_usize);
                swap_processed_input[swap_index] = true;
                swap_input.swap(initial_swap_input, swap_index);
                exhaustive_permutations(swap_input, swap_processed_input)
            })
            .flatten()
            .collect::<Vec<Vec<usize>>>()
    }
}

// TODO - Add Cacher
pub(crate) fn find_all_permutations(input: Vec<usize>) -> Vec<Vec<usize>> {
    let processed_input: Vec<bool> = vec![false; input.len()];
    exhaustive_permutations(input, processed_input)
}
