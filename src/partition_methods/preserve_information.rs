use frequency::Frequency;
use frequency_hashmap::HashMapFrequency;
use std::collections::HashMap;

pub(crate) fn create_category_metrics(group_definition: Vec<usize>, element_value: Vec<f32>) -> CategoryMetrics {
    let group_frequency: HashMapFrequency<usize, usize> =
        group_definition.clone().into_iter().collect();
    let mut group_mean: HashMap<usize, f32> = HashMap::new();
    let n_groups: f32 = group_frequency.items().len() as f32;
    for (group, element) in
        group_definition
            .into_iter()
            .zip(element_value.into_iter()) {
        if let Some(x) = group_mean.get_mut(&group) {
            *x += element / n_groups
        } else {
            group_mean.insert(group, element / n_groups);
        }
    }
    CategoryMetrics {
        group: group_frequency.items().copied().collect::<Vec<usize>>(),
        elements_per_group: group_frequency.counts().copied().collect::<Vec<usize>>(),
        group_mean: group_frequency.items().map(|group| group_mean[group]).collect(),
    }
}

#[derive(Clone)]
pub struct CategoryMetrics {
    pub group: Vec<usize>,
    pub elements_per_group: Vec<usize>,
    pub group_mean: Vec<f32>,
}

fn exhaustive_segments(input: Vec<usize>, groups: usize) -> Vec<Vec<Vec<usize>>> {
    if groups == 1 {
        return vec![vec![input.clone()]];
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

pub(crate) fn segments(
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
        let strata_mean: f32 = index_group.clone().into_iter().fold(0.0_f32, |acc, index| {
            acc + data.elements_per_group[index] as f32 * data.group_mean[index]
        }) / (index_group.clone().len() as f32);

        acc + index_group.iter().fold(0.0_f32, |acc, index| {
            acc + (data.elements_per_group[*index] as f32
                * (data.group_mean[*index] - strata_mean)
                * (data.group_mean[*index] - strata_mean))
        })
    })
}
