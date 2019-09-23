use std::cmp::Ordering;
use std::ops::Add;
use crate::permutations::exhaustive_finder::find_all_permutations;

// At each Node, there is either a single element that indicates a KK heuristic
// and an index map or a vector of elements
#[derive(Clone, Debug)]
enum NodeBranch {
    More(Vec<NodeBranch>),
    Score(Vec<Vec<usize>>, f32),
    Pruned()
}

#[derive(Clone, Debug)]
pub(crate) struct EvenSizedBinNodeTree {
    data: NodeBranch,
}

#[derive(Clone, Debug)]
struct Tuple {
    data: Vec<f32>,
    bin_assignments: Vec<Vec<usize>>,
}

impl Tuple {
    fn sum(&self) -> f32 {
        self.data.clone().into_iter().fold(0.0_f32, f32::add)
    }
}

impl PartialOrd for Tuple {
    fn partial_cmp(&self, other: &Tuple) -> Option<Ordering> {
        self.sum().partial_cmp(&other.sum())
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.sum() != other.sum()
    }
}

/// Multiway Partitioning of Evenly Sized Bins - Complete Karmarkar-Karp Algorithm (CKK)
/// Written with guidance from https://www.ijcai.org/Proceedings/09/Papers/096.pdf
fn create_multi_way_partition_tree(
    tuple: Vec<Tuple>,
    naive_estimate: f32,
    partitions: usize,
) -> NodeBranch {
    /*
        Create all permutations of tuples using a permutation finder, then resolve to create
        a vector of possible permutations at each layer, with intention to prune to find an ideal
        case.
    */
    if tuple.len() == 1 {
        let final_tuple = tuple.clone().pop().unwrap();
        let score = final_tuple.data.into_iter().fold(0.0_f32, f32::add);
        NodeBranch::Score(
            final_tuple.bin_assignments,
            score,
        )
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
                if (permutation_tuple.clone().into_iter().fold(std::f32::MIN, f32::max) -
                    tuple.clone()[2..].to_owned().iter().fold(0.0_f32, |acc, tuple_elem| {
                        acc + tuple_elem.sum()
                    }) / partitions as f32).abs() > naive_estimate {
                    NodeBranch::Pruned()
                } else {
                    let mut permutation_tuple: Vec<Tuple> = {
                        let mut extended_tuple: Vec<Tuple> = vec![Tuple {
                            data: permutation_tuple,
                            bin_assignments: highest_tuple_bin_assignment,
                        }];
                        let existing_tuples: Vec<Tuple> =
                            tuple.clone()[2..].to_owned().into_iter().collect();
                        extended_tuple.extend(existing_tuples);
                        extended_tuple
                    };
                    permutation_tuple
                        .sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

                    create_multi_way_partition_tree(
                        permutation_tuple,
                        naive_estimate,
                        partitions,
                    )
                }
            })
            .filter(|branch| {
                match branch {
                    NodeBranch::Pruned() => false,
                    _ => true
                }
            })
            .collect::<Vec<NodeBranch>>();
        NodeBranch::More(result_tuple)
    }
}

impl EvenSizedBinNodeTree {
    fn new(data: Vec<f32>, partitions: usize) -> EvenSizedBinNodeTree {
        let naive_estimate = data.clone().into_iter().fold(0.0_f32, f32::add)
            / (data.len() as f32 / partitions  as f32);
        let mut binary_tree_data = data.clone();
        let mut index: usize = 0_usize;
        // Forward Sort with intention to pop for reverse ordering
        binary_tree_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut tuple: Vec<Tuple> = Vec::with_capacity(data.len());
        while let Some(initial_state_element) = binary_tree_data.pop() {
            // Could replace this with a Binary Heap, as it'll ensure
            // ordering on the internal elements for the tuple
            let mut ins_vec = vec![0_f32; partitions];
            ins_vec[0] = initial_state_element;
            let mut bin_assignments: Vec<Vec<usize>> = vec![vec![]; partitions];
            bin_assignments[0].push(index);
            tuple.push(Tuple {
                data: ins_vec,
                bin_assignments,
            });
            index += 1_usize;
        }
        EvenSizedBinNodeTree {
            data: create_multi_way_partition_tree(tuple.clone(), naive_estimate, partitions),
        }
    }

    fn flatten_node_tree(self) -> Vec<(Vec<Vec<usize>>, f32)> {
        fn flatten_node_point(initial_point : NodeBranch) -> Vec<NodeBranch> {
            match initial_point {
                NodeBranch::More(point) => {
                    point.into_iter().map(flatten_node_point).flatten().collect::<Vec<NodeBranch>>()
                },
                NodeBranch::Score(bin, score) => vec![NodeBranch::Score(bin, score)],
                _ => unimplemented!()
            }
        };
        flatten_node_point(self.data)
            .iter()
            .map(|point| {
                match point {
                    NodeBranch::Score(bin, score) => {(bin.clone(), *score)},
                    _ => unreachable!()
                }
            })
            .collect::<Vec<(Vec<Vec<usize>>, f32)>>()
    }
}

pub fn multi_way_partition(data: Vec<f32>, partitions: usize) -> (Vec<Vec<usize>>, f32) {
    let mut group = EvenSizedBinNodeTree::new(data, partitions).flatten_node_tree();
    group.sort_by(|a,b| {
        (b.1).partial_cmp(&(a.1)).unwrap_or(std::cmp::Ordering::Equal)
    });
    group.pop().unwrap()
}