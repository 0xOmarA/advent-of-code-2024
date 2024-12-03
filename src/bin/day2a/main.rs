use itertools::Itertools;

fn main() {
    let reports = load_reports();
    let valid_reports = reports
        .map(|report| {
            let mut contains_positive = false;
            let mut contains_negative = false;
            let differences = report
                .tuple_windows()
                .map(|(item1, item2)| item2 as isize - item1 as isize);
            for difference in differences {
                if difference.is_positive() {
                    contains_positive = true;
                } else if difference.is_negative() {
                    contains_negative = true;
                }
                if !matches!(difference.abs(), 1..=3)
                    || contains_positive && contains_negative
                {
                    return false;
                }
            }
            true
        })
        .inspect(|value| println!("{value}"))
        .filter(|value| *value)
        .collect::<Vec<_>>()
        .len();

    println!("Valid reports: {valid_reports}")
}

pub fn load_reports() -> impl Iterator<Item = impl Iterator<Item = usize>> {
    unsafe {
        let inputs = include_str!("inputs.txt");
        inputs.split('\n').map(|line| {
            line.split(' ')
                .map(|item| item.parse::<usize>().unwrap_unchecked())
        })
    }
}
