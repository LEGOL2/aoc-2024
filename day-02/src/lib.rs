pub fn count_safe_reports(lines: Vec<String>) -> i32 {
    let numbers = convert(lines);
    numbers
        .iter()
        .map(|row| verify_row(row))
        .filter(|r| *r)
        .count() as i32
}

pub fn count_safe_reports_with_reverification(lines: Vec<String>) -> i32 {
    let numbers = convert(lines);
    let mut safe_reports = 0;

    for row in numbers {
        if verify_row(&row) {
            safe_reports += 1;
        } else {
            for i in 0..row.len() {
                let mut modified = row.clone();
                modified.remove(i);
                if verify_row(&modified) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    safe_reports
}

fn verify_row(row: &Vec<i32>) -> bool {
    let diffs = row
        .as_slice()
        .windows(2)
        .map(|s| s[0] - s[1])
        .collect::<Vec<i32>>();
    if diffs.iter().any(|n| *n == 0 || i32::abs(*n) > 3) {
        return false;
    }

    if diffs.iter().all(|n| *n < 0) || diffs.iter().all(|n| *n > 0) {
        return true;
    }

    false
}

fn convert(lines: Vec<String>) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_file;

    #[test]
    fn basic_example_part1() {
        let lines = read_file("test_input.txt").unwrap();
        assert_eq!(count_safe_reports(lines), 2);
    }

    #[test]
    fn basic_example_part2() {
        let lines = read_file("test_input.txt").unwrap();
        assert_eq!(count_safe_reports_with_reverification(lines), 4);
    }
}
