use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut top_three = [u32::MIN; 3];

    for calories in input.iter() {
        let mut calories = *calories;

        for top_value in top_three.iter_mut() {
            if calories > *top_value {
                std::mem::swap(top_value, &mut calories);
            }
        }
    }

    top_three.iter().sum::<u32>().into()
}
