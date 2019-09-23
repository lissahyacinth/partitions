pub mod partition_methods;
pub (crate) mod permutations;

use crate::partition_methods::multiway_partition::{multi_way_partition};

fn evenly_partition_input(input: Vec<f32>, partitions: usize) -> Vec<Vec<usize>> {
    let (groups, _) = multi_way_partition(input, partitions);
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_partition() {
        let input : Vec<f32> = vec![400_f32, 50_f32, 250_f32, 300_f32];
        assert_eq!(evenly_partition_input(input, 2),
        vec![
            vec![2,1],
            vec![0,3]
        ]);
    }
}
