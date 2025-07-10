use itertools::Itertools;

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    match (0..=items.len())
        .flat_map(|l| items.iter().combinations(l))
        .filter(|combination| {
            // Sum of weight should be less than max_weight
            let mut total_weight: u32 = 0;
            for &item in combination {
                total_weight += item.weight;
            }

            total_weight <= max_weight && total_weight > 0
        })
        .map(|combination| {
            // For each combination, compute the total value
            combination.iter().map(|&item| item.value).sum::<u32>()
        })
        .collect::<Vec<u32>>()
        .iter()
        .max()
    {
        Some(max) => *max,
        None => 0,
    }
}
