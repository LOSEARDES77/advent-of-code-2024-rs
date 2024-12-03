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
        let mut es_segura = true;
        let line_data = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut buf = line_data[0];
        let mut ascent = None;
        let mut fails = 0;
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
                        fails += 1;
                    }
                }
            }

            if let Some(val) = ascent {
                if val {
                    let a = i - buf;
                    if !(1..=3).contains(&a) {
                        es_segura = false;
                        fails += 1;
                    }
                } else {
                    let a = buf - i;
                    if !(1..=3).contains(&a) {
                        es_segura = false;
                        fails += 1;
                    }
                }
            }

            buf = *i;
        }
        if es_segura || fails <= 1 {
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
}
