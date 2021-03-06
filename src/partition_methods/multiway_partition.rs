use std::cmp::Ordering;
use std::ops::Add;

use crate::permutations::exhaustive_finder::find_all_permutations;

// At each Node, there is either a single element that indicates a KK heuristic
// and an index map or a vector of elements
#[derive(Clone, Debug)]
enum NodeBranch {
    More(Vec<NodeBranch>),
    Score(Vec<Vec<usize>>, f32),
    Pruned(),
}

#[derive(Clone, Debug)]
pub(crate) struct EvenSizedBinNodeTree {
    data: NodeBranch,
    pub(crate) sort_order: Vec<usize>
}

#[derive(Clone, Debug)]
struct KarmarkerTuple {
    data: Vec<f32>,
    bin_assignments: Vec<Vec<usize>>,
}

impl KarmarkerTuple {
    fn sum(&self) -> f32 {
        self.data.clone().into_iter().fold(0.0_f32, f32::add)
    }
}

impl PartialOrd for KarmarkerTuple {
    fn partial_cmp(&self, other: &KarmarkerTuple) -> Option<Ordering> {
        self.sum().partial_cmp(&other.sum())
    }
}

impl PartialEq for KarmarkerTuple {
    fn eq(&self, other: &KarmarkerTuple) -> bool {
        self.sum() != other.sum()
    }
}

/// Multiway Partitioning of Evenly Sized Bins - Complete Karmarkar-Karp Algorithm (CKK)
/// Written with guidance from https://www.ijcai.org/Proceedings/09/Papers/096.pdf
fn create_multi_way_partition_tree(
    tuple: Vec<KarmarkerTuple>,
    naive_estimate: f32,
    partitions: usize,
) -> NodeBranch {
    /*
        Create all permutations of tuples using a permutation finder, then resolve to create
        a vector of possible permutations at each layer, with intention to prune to find an ideal
        case.
    */
    if tuple.len() == 1 {
        let bin_assignments = tuple[0].bin_assignments.clone();
        let score = tuple[0].data.clone().into_iter().fold(0.0_f32, f32::add);
        NodeBranch::Score(bin_assignments, score)
    } else {
        let permutation_input = (0..partitions).collect::<Vec<usize>>();
        let permutations = find_all_permutations(permutation_input);
        // Tuple Externals are defined as pre-sorted from largest to smallest.
        let highest_tuple = tuple[0].data.clone();
        let second_highest_tuple = tuple[1].data.clone();
        let result_tuple = permutations
            .into_iter()
            .map(|permutation_indices| {
                // For each permutation, add together each index, i.e.
                // A + P[0], B + P[1], C + P[2], ..., Z + [P[N]]
                let mut permutation_tuple = (0_usize..partitions)
                    .map(|index| {
                        highest_tuple[index] + second_highest_tuple[permutation_indices[index]]
                    })
                    .collect::<Vec<f32>>();

                let mut highest_tuple_bin_assignment = tuple[0].bin_assignments.clone();
                // Modify the Assigment for the Highest Tuple
                let second_highest_tuple_bin_assignment = tuple[1].bin_assignments.clone();
                for (index, bin) in highest_tuple_bin_assignment.iter_mut().enumerate() {
                    bin.extend(
                        second_highest_tuple_bin_assignment[permutation_indices[index]].clone(),
                    )
                }
                // Normalisation step - minus the lowest value in the triple from each of the sets.
                let min_point = permutation_tuple
                    .clone()
                    .into_iter()
                    .fold(std::f32::MAX, f32::min);
                for item in permutation_tuple.iter_mut() {
                    *item -= min_point
                }

                // After Normalisation, return early if the remaining elements cannot be placed
                // in an ideal situation better than a naive solve
                if (permutation_tuple
                    .clone()
                    .into_iter()
                    .fold(std::f32::MIN, f32::max)
                    - tuple.clone()[2..]
                    .to_owned()
                    .iter()
                    .fold(0.0_f32, |acc, tuple_elem| acc + tuple_elem.sum())
                    / partitions as f32)
                    .abs()
                    > naive_estimate
                {
                    NodeBranch::Pruned()
                } else {
                    let mut permutation_tuple: Vec<KarmarkerTuple> = {
                        let mut extended_tuple: Vec<KarmarkerTuple> = vec![KarmarkerTuple {
                            data: permutation_tuple,
                            bin_assignments: highest_tuple_bin_assignment,
                        }];
                        let existing_tuples: Vec<KarmarkerTuple> =
                            tuple.clone()[2..].to_owned().into_iter().collect();
                        extended_tuple.extend(existing_tuples);
                        extended_tuple
                    };
                    permutation_tuple
                        .sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

                    create_multi_way_partition_tree(permutation_tuple, naive_estimate, partitions)
                }
            })
            .filter(|branch| match branch {
                NodeBranch::Pruned() => false,
                _ => true,
            })
            .collect::<Vec<NodeBranch>>();
        NodeBranch::More(result_tuple)
    }
}

impl EvenSizedBinNodeTree {
    pub(crate) fn new(data: Vec<f32>, partitions: usize) -> EvenSizedBinNodeTree {
        let naive_estimate = data.clone().into_iter().fold(0.0_f32, f32::add)
            / (partitions as f32);

        let mut binary_tree_data = data
            .clone()
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, f32)>>();

        // Forward Sort with intention to pop for reverse ordering
        binary_tree_data
            .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut sort_order: Vec<usize> = binary_tree_data.clone().into_iter().map(|(index, _)| index).collect();
        sort_order.reverse();
        let mut binary_tree_data: Vec<f32> = binary_tree_data.into_iter().map(|(_, elem)| elem).collect();

        let mut tuple: Vec<KarmarkerTuple> = Vec::with_capacity(data.len());
        let mut index: usize = 0_usize;
        while let Some(initial_state_element) = binary_tree_data.pop() {
            // Could replace this with a Binary Heap, as it'll ensure
            // ordering on the internal elements for the tuple
            let mut ins_vec = vec![0_f32; partitions];
            ins_vec[0] = initial_state_element;
            let mut bin_assignments: Vec<Vec<usize>> = vec![vec![]; partitions];
            bin_assignments[0].push(index);
            tuple.push(KarmarkerTuple {
                data: ins_vec,
                bin_assignments,
            });
            index += 1_usize;
        }
        EvenSizedBinNodeTree {
            data: create_multi_way_partition_tree(tuple, naive_estimate, partitions),
            sort_order
        }
    }

    pub(crate) fn flatten_node_tree(self) -> Vec<(Vec<Vec<usize>>, f32)> {
        fn flatten_node_point(initial_point: NodeBranch) -> Vec<NodeBranch> {
            match initial_point {
                NodeBranch::More(point) => point
                    .into_iter()
                    .map(flatten_node_point)
                    .flatten()
                    .collect::<Vec<NodeBranch>>(),
                NodeBranch::Score(bin, score) => vec![NodeBranch::Score(bin, score)],
                _ => unimplemented!(),
            }
        };
        flatten_node_point(self.data)
            .iter()
            .map(|point| match point {
                NodeBranch::Score(bin, score) => (bin.clone(), *score),
                _ => unreachable!(),
            })
            .collect::<Vec<(Vec<Vec<usize>>, f32)>>()
    }
}

