use frequency_hashmap::HashMapFrequency;
use frequency::Frequency;

#[derive(Clone, Debug)]
pub struct CategoryMetrics {
    pub group: Vec<usize>,
    pub elements_per_group: Vec<usize>,
    pub group_mean: Vec<f32>
}

pub(crate) trait CategoryMetricInfo {
    fn metrics(self, group_definition: Vec<usize>) -> CategoryMetrics;
}

impl CategoryMetricInfo for Vec<f32> {
    fn metrics(self, group_definition: Vec<usize>) -> CategoryMetrics {
        let group_frequency: HashMapFrequency<usize, usize> =
            group_definition.clone().into_iter().collect();
        let n_groups: usize = group_frequency.items().len();
        assert_eq!(n_groups - 1, *group_definition.iter().max().unwrap());

        let mut group_mean = vec![0_f32; n_groups];
        for (group, element) in
        group_definition
            .into_iter()
            .zip(self.into_iter()) {
            group_mean[group] += element;
        }

        CategoryMetrics {
            group: group_frequency.items().copied().collect::<Vec<usize>>(),
            elements_per_group: group_frequency.counts().copied().collect::<Vec<usize>>(),
            group_mean: group_frequency.items().map(|group| {
                group_mean[*group] / group_frequency.count(group) as f32
            }).collect(),
        }
    }
}

impl CategoryMetricInfo for Vec<bool> {
    fn metrics(self, group_definition: Vec<usize>) -> CategoryMetrics {
        let group_frequency: HashMapFrequency<usize, usize> =
            group_definition.clone().into_iter().collect();

        let n_groups: usize = group_frequency.items().len();
        assert_eq!(n_groups - 1, *group_definition.iter().max().unwrap());

        let mut group_true_probability = vec![0f32; n_groups];
        for (group, element) in
        group_definition
            .into_iter()
            .zip(self.into_iter()) {
            group_true_probability[group] += (element as i32) as f32;
        }

        let mut groups = group_frequency.items().copied().collect::<Vec<usize>>();

        CategoryMetrics {
            group: groups,
            elements_per_group: group_frequency.counts().copied().collect::<Vec<usize>>(),
            group_mean: group_frequency.items().map(|group| {
                group_true_probability[*group] / group_frequency.count(group) as f32
            }).collect(),
        }
    }
}