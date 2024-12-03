use regex::Regex;

pub fn solve(input: &str) -> String {
    let re = Regex::new(r"^([0-9]{1,3},[0-9]{1,3})$").unwrap();
    let mut sum = 0;
    for chunk in input.split("mul(").skip(1) {
        if let Some(mult_data) = chunk.split(')').next() {
            if re.is_match(mult_data) {
                let (x, y) = mult_data.split_once(',').unwrap();
                let res = x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
                sum += res;
            }
        }
    }

    sum.to_string()
}

fn split_with_regex(input: &str, separator: &str) -> Vec<String> {
    let re = Regex::new(separator).unwrap();
    let mut result = Vec::new();
    let mut last_end = 0;

    for mat in re.find_iter(input) {
        // Push the part before the match
        if mat.start() > last_end {
            result.push(input[last_end..mat.start()].to_string());
        }
        // Push the matched separator as a part of the next chunk
        result.push(input[mat.start()..mat.end()].to_string());
        last_end = mat.end();
    }

    // Push the remaining part after the last match
    if last_end < input.len() {
        result.push(input[last_end..].to_string());
    }

    result
}

pub fn solve_p2(input: &str) -> String {
    let re = Regex::new(r"^([0-9]{1,3}),([0-9]{1,3})$").unwrap();
    let mut enabled = true;
    let mut sum = 0;

    for section in split_with_regex(input, r"(do\(\)|don't\(\))").iter() {
        if enabled {
            if section != "do()" && section != "don't()" {
                for chunk in section.split("mul(").skip(1) {
                    if let Some(mult_data) = chunk.split(')').next() {
                        if re.is_match(mult_data) {
                            let (x, y) = mult_data.split_once(',').unwrap();
                            let res = x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
                            sum += res;
                        }
                    }
                }
            } else if section == "don't()" {
                enabled = false;
                continue;
            }
        } else if section == "do()" {
            enabled = true;
            continue;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let res = solve(input);
        assert_eq!(res, "161")
    }

    #[test]
    fn test_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let res = solve_p2(input);
        assert_eq!(res, "48");
    }
}
