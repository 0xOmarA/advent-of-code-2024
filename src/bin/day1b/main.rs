use std::collections::*;

/// In the solution of this problem I'm using some assumptions based on what I
/// observed from the given numbers.
///
/// 1. The numbers can be represented as an [`i32`].
/// 2. We will never fail in parsing the numbers - they're all valid.
/// 3. All lines contain two numbers.
fn main() {
    // The inputs are small enough that it's okay for us to just read them all
    // at one time and include them in the binary as a static dependency.
    let inputs = include_str!("inputs.txt");
    let (left_list, right_list) = unsafe {
        inputs
            .split('\n')
            .map(|line| {
                let mut splitted_line = line.split("   ");
                let item1 = splitted_line
                    .next()
                    .unwrap_unchecked()
                    .parse::<i32>()
                    .unwrap_unchecked();
                let item2 = splitted_line
                    .next()
                    .unwrap_unchecked()
                    .parse::<i32>()
                    .unwrap_unchecked();
                (item1, item2)
            })
            .unzip::<_, _, Vec<_>, Vec<_>>()
    };

    let repetitions_in_right_list =
        right_list
            .iter()
            .fold(HashMap::<i32, usize>::new(), |mut map, number| {
                *map.entry(*number).or_default() += 1;
                map
            });
    let similarity_score = left_list.iter().fold(0usize, |acc, item| {
        acc + (*item as usize * repetitions_in_right_list.get(item).copied().unwrap_or(0))
    });
    println!("Similarity Score: {similarity_score}")
}
