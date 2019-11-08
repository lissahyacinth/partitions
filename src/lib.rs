use crate::partition_methods::multiway_partition::multiway_partition;
use crate::partition_methods::preserve_information::{create_category_metrics, segments};

pub mod partition_methods;
pub(crate) mod permutations;

/// Group Vector into Even Partitions
pub fn evenly_partition_input(input: Vec<f32>, partitions: usize) -> Vec<Vec<usize>> {
    multiway_partition(input, partitions)
}

/// Identify Highest Information Preserving Segments of a Numeric Group
/// Using Fisher Score (Closest Sample Standard Deviations to initial Population), identify
/// the best segments (groups of groups) to preserve.
///
/// # Returns
/// Single Best Partitions of Vec<Group - Vec<Group ID>>
pub fn bin_by_preserved_information(
    group_column: Vec<usize>,
    element_column: Vec<f32>,
) -> Vec<Vec<usize>> {
    let group_descriptors = create_category_metrics(
        group_column,
        element_column);
    let target_bins: usize = std::cmp::max(2_usize, (group_descriptors.group.len() as f32).cbrt() as usize);
    segments(&group_descriptors.group, &group_descriptors, target_bins)
        .pop()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_partitioning() {
        let input: Vec<f32> = vec![400_f32, 300_f32, 50_f32, 300_f32, 70_f32, 30_f32];
        assert_eq!(
            evenly_partition_input(input, 2),
            vec![
                vec![1, 3],
                vec![0, 4, 2, 5]
            ]
        );
    }

    #[test]
    fn preserved_info_partitioning() {
        let input_categories: Vec<usize> = vec![0, 0, 1, 1, 2, 2];
        let input: Vec<f32> = vec![
            400_f32, 50_f32, // 450
            250_f32, 300_f32, // 550
            100_f32, 400_f32]; // 500
        assert_eq!(
            bin_by_preserved_information(input_categories, input),
            vec![
                vec![1],
                vec![0, 2]
            ]
        );
    }
}
