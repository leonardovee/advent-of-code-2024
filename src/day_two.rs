use std::fs::read_to_string;

pub fn solve() -> i32 {
    let input = read_to_string("./input/day_two").unwrap();
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|value| {
            let mut is_safe = is_report_safe(&value);
            if !is_safe {
                is_safe = can_save(&value)
            }
            is_safe
        })
        .map(|value| if value { return 1 } else { return 0 })
        .sum()
}

fn can_save(nums: &Vec<i32>) -> bool {
    for (i, _) in nums.iter().enumerate() {
        let mut new_nums = nums.clone();
        new_nums.remove(i);
        let is_safe = is_report_safe(&new_nums);
        if is_safe {
            return true;
        }
    }
    false
}

fn is_report_safe(nums: &Vec<i32>) -> bool {
    let is_sorted = is_sorted(&nums);

    // do some sliding window
    let mut safe_reports: i32 = 0;
    for window in nums.windows(2) {
        // unsafe because is neither an increase or decrease
        if window[0] == window[1] {
            return false;
        }
        // safe because the levels are increasing by 1, 2 or 3
        let diff = (window[0] - window[1]).abs();
        if diff >= 1 && diff <= 3 {
            safe_reports += 1;
        }
    }

    is_sorted && safe_reports >= nums.len() as i32 - 1
}

fn is_sorted(nums: &Vec<i32>) -> bool {
    let asc = nums.iter().is_sorted();
    let desc = nums.iter().rev().is_sorted();
    asc || desc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(), 563);
    }
}
