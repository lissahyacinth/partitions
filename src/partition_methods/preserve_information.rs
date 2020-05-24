use crate::partition_methods::category_metrics::CategoryMetrics;

fn exhaustive_segments(input: Vec<usize>, groups: usize) -> Vec<Vec<Vec<usize>>> {
    if groups == 1 {
        return vec![vec![input]];
    } else {
        (1..=(input.len() - groups + 1))
            .map(|i| {
                let head = input[0..i].to_vec();
                exhaustive_segments(input[i..input.len()].to_vec(), groups - 1)
                    .iter()
                    .map(|tail| {
                        let mut elements = tail.clone();
                        elements.push(head.clone());
                        elements.reverse();
                        elements
                    })
                    .collect::<Vec<Vec<Vec<usize>>>>()
            })
            .flatten()
            .collect()
    }
}

pub fn segments(
    group_id: &[usize],
    group_descriptors: &CategoryMetrics,
    groups: usize,
) -> Vec<Vec<Vec<usize>>> {
    if group_id.len() <= 10 {
        vec![
            exhaustive_segments(group_id.to_vec(), groups)
                .into_iter()
                .map(|segment| (segment.clone(), fisher_score(segment, &group_descriptors)))
                .min_by(|x, y| (x.1).partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap()
                .0,
        ]
    } else {
        let lhs_groups = groups / 2;
        let half_index: usize = ((group_id.len() as f32) / 2.0).round() as usize;
        let mut left = segments(&group_id[0..half_index], &group_descriptors, groups / 2)
            .pop()
            .unwrap();
        let right = segments(
            &group_id[half_index..],
            &group_descriptors,
            groups - lhs_groups,
        )
            .pop()
            .unwrap();
        left.extend(right);
        return vec![left];
    }
}

fn fisher_score(indexes: Vec<Vec<usize>>, data: &CategoryMetrics) -> f32 {
    indexes.iter().fold(0.0, |acc, index_group| {
        let strata_elements = index_group.clone().into_iter().fold(0f32, |acc, index| acc+data.elements_per_group[index] as f32);
        // Mean of each index group
        let strata_mean: f32 = index_group.clone().into_iter().fold(0.0_f32, |acc, index| {
            data.group_mean[index].mul_add(
                data.elements_per_group[index] as f32,
                acc,
            )
        }) / strata_elements;
        // Difference of each index group from the mean squared, plus the number of elements per group?
        acc + index_group.iter().fold(0.0_f32, |acc, index| {
            acc + (data.group_mean[*index] - strata_mean) *
                (data.group_mean[*index] - strata_mean) *
                (data.elements_per_group[*index] as f32)
        })
    })
}
