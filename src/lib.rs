use crate::partition_methods::multiway_partition::EvenSizedBinNodeTree;
use crate::partition_methods::preserve_information::{create_category_metrics, segments};

pub mod partition_methods;
pub(crate) mod permutations;

/// Partition Elements into Evenly Summed Groups
/// Uses a complete method of the Karmarkar-Karp differencing algorithm to best partition elements
/// into a specified number of partitions.
pub fn multiway_partition(data: Vec<f32>, partitions: usize) -> Vec<Vec<usize>> {
    let node_tree: EvenSizedBinNodeTree = EvenSizedBinNodeTree::new(data, partitions);
    let sort_order: Vec<usize> = node_tree.sort_order.clone();
    let mut group = node_tree.flatten_node_tree();

    group.sort_by(|a, b| {
        (b.1)
            .partial_cmp(&(a.1))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let ideal_sort = group.pop().unwrap();
    // Reorder Sort from Max->Min to Original Order
    ideal_sort.0
        .into_iter()
        .map(|element_group| {
            element_group.into_iter().map(|item| {
                sort_order[item]
            }).collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

/// Identify Highest Information Preserving Segments of a Numeric Group
/// Using Fisher Score (Closest Sample Standard Deviations to initial Population), identify
/// the best segments (groups of groups) to preserve.
///
/// # Returns
/// Single Best Partitions of Vec<Group - Vec<Group ID>>
pub fn partition_by_information(
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
    use crate::multiway_partition;
    use crate::partition_by_information;

    #[test]
    fn even_partitioning() {
        let input: Vec<f32> = vec![400_f32, 300_f32, 50_f32, 300_f32, 70_f32, 30_f32];
        assert_eq!(
            multiway_partition(input, 2),
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
            partition_by_information(input_categories, input),
            vec![
                vec![1],
                vec![0, 2]
            ]
        );
    }
}
