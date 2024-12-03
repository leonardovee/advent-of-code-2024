use std::fs::read_to_string;

pub fn solve() -> i32 {
    let input = read_to_string("./input/day_one").unwrap();

    let (left, right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok());
            if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
                Some((left, right))
            } else {
                None
            }
        })
        .unzip();

    left.iter()
        .map(|value| right.iter().filter(|x| *x == value).count() as i32 * value)
        .sum()

    //    left.sort();
    //    right.sort();
    //
    //    left.iter()
    //        .zip(right.iter_mut())
    //        .map(|(a, b)| (*a - *b).abs())
    //        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(), 23981443);
    }
}
