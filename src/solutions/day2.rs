use std::cmp::Ordering;

pub fn solve(input: &str) -> String {
    let data = input.lines().collect::<Vec<_>>();
    let mut sum = 0;
    for line in data {
        let mut es_segura = true;
        let line_data = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut buf = line_data[0];
        let mut ascent = None;
        for (idx, i) in line_data[1..].iter().enumerate() {
            if idx == 0 {
                match buf.cmp(i) {
                    Ordering::Less => {
                        ascent = Some(true);
                    }
                    Ordering::Greater => {
                        ascent = Some(false);
                    }
                    Ordering::Equal => {
                        es_segura = false;
                        break;
                    }
                }
            }

            if let Some(val) = ascent {
                if val {
                    let a = i - buf;
                    if !(1..=3).contains(&a) {
                        es_segura = false;
                        break;
                    }
                } else {
                    let a = buf - i;
                    if !(1..=3).contains(&a) {
                        es_segura = false;
                        break;
                    }
                }
            }

            buf = *i;
        }
        if es_segura {
            sum += 1;
        }
    }
    sum.to_string()
}

pub fn solve_p2(input: &str) -> String {
    let data = input.lines().collect::<Vec<_>>();
    let mut sum = 0;

    for line in data {
        let line_data = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // Function to check if a sequence is safe
        let is_safe = |seq: &[i32]| -> bool {
            if seq.len() < 2 {
                return true; // A single level or empty sequence is trivially safe
            }

            let mut buf = seq[0];
            let mut ascent = None;

            for &i in &seq[1..] {
                match buf.cmp(&i) {
                    Ordering::Less => {
                        if ascent == Some(false) {
                            return false; // Switch in direction
                        }
                        ascent = Some(true);
                        if i - buf < 1 || i - buf > 3 {
                            return false; // Invalid difference
                        }
                    }
                    Ordering::Greater => {
                        if ascent == Some(true) {
                            return false; // Switch in direction
                        }
                        ascent = Some(false);
                        if buf - i < 1 || buf - i > 3 {
                            return false; // Invalid difference
                        }
                    }
                    Ordering::Equal => return false, // Equal levels are not allowed
                }
                buf = i;
            }
            true
        };

        // Check if the report is safe without changes
        if is_safe(&line_data) {
            sum += 1;
            continue;
        }

        // Check if removing one level makes it safe
        let mut safe_with_removal = false;
        for i in 0..line_data.len() {
            let mut modified = line_data.clone();
            modified.remove(i);
            if is_safe(&modified) {
                safe_with_removal = true;
                break;
            }
        }

        if safe_with_removal {
            sum += 1;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let result = solve(input);
        println!("{}", result);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_2() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let result = solve_p2(input);
        println!("{}", result);
        assert_eq!(result, "4");
    }
}
