#[allow(dead_code)]
fn football_partition(mut data: Vec<f32>, ideal_group_length: usize) -> Vec<Vec<f32>> {
    data.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let ideal_group_value: f32 =
        data.clone().iter().fold(0.0_f32, |acc, elem| acc + *elem) / ideal_group_length as f32;
    let mut segmentations: Vec<Vec<f32>> = Vec::with_capacity(ideal_group_length);
    for _ in 0..ideal_group_length {
        segmentations.push(vec![data.pop().unwrap()])
    }
    while let Some(element) = data.pop() {
        // Find segmentation that would become closest to ideal
        let mut closest_segment = segmentations
            .iter()
            .enumerate()
            .map(|(index, element_group)| {
                (
                    index,
                    (ideal_group_value
                        - (element_group.clone().into_iter().sum::<f32>() + element)),
                )
            })
            .collect::<Vec<(usize, f32)>>();

        closest_segment.sort_by(|(_, diff), (_, other_diff)| {
            diff.partial_cmp(other_diff)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let (segmentation_index, _) = closest_segment.pop().unwrap();
        segmentations[segmentation_index].push(element);
    }
    segmentations
}

#[allow(dead_code)]
fn score_segmentations(elements: Vec<Vec<f32>>, point_average: f32) -> f32 {
    elements
        .iter()
        .map(|group| {
            let mut total: f32 = 0.0_f32;
            for group_elem in group.iter() {
                total += *group_elem
            }
            (total - point_average).abs()
        })
        .fold(0.0_f32, |acc: f32, elem: f32| acc + elem)
}
