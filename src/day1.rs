use std::cmp::{max, min, Ordering};

pub fn solve(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    let mut right_col: Vec<u32> = Vec::new();
    let mut left_col: Vec<u32> = Vec::new();
    for line in lines {
        let (left, right) = line.split_once("   ").unwrap();
        right_col.push(right.parse().unwrap());
        left_col.push(left.parse().unwrap());
    }

    left_col = quick_sort(left_col);
    right_col = quick_sort(right_col);
    let mut sum = 0;
    for i in 0..left_col.len() {
        let distance = max(right_col[i], left_col[i]) - min(right_col[i], left_col[i]);
        sum += distance;
    }
    sum.to_string()
}

pub fn quick_sort(arr: Vec<u32>) -> Vec<u32> {
    if arr.len() <= 1 {
        return arr;
    }
    let pivot = arr[arr.len() / 2];
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut middle = Vec::new();
    for x in arr {
        match x.cmp(&pivot) {
            Ordering::Less => left.push(x),
            Ordering::Greater => right.push(x),
            Ordering::Equal => middle.push(x),
        }
    }
    let mut sorted = quick_sort(left);
    sorted.append(&mut middle);
    sorted.append(&mut quick_sort(right));
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        let result = solve(input);
        println!("{}", result);
        assert_eq!(result, "11");
    }
}
