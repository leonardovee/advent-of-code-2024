use std::fs::read_to_string;

use regex::Regex;

pub fn solve() -> i64 {
    let input = read_to_string("./input/day_three").unwrap();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;
    for cap in re.captures_iter(&input) {
        let x: i64 = cap[1].parse().unwrap();
        let y: i64 = cap[2].parse().unwrap();

        result += x * y;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(), 173517243);
    }
}
