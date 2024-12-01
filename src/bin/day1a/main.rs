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
    let (mut vec1, mut vec2) = unsafe {
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
    vec1.sort();
    vec2.sort();

    let total_distance = vec1
        .into_iter()
        .zip(vec2)
        .map(|(item1, item2)| (item1 - item2).abs())
        .sum::<i32>();
    println!("{total_distance}");
}
