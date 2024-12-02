pub fn count_safe_reports(lines: Vec<String>) -> i32 {
    let numbers = convert(lines);
    let mut safe_reports = 0;

    for row in numbers {
        let mut state = State::UNKNOWN;
        let len = row.len();
        let mut safe = true;

        for i in 0..len - 1 {
            let diff = row[i] - row[i + 1];
            if diff == 0 {
                safe = false;
                break;
            }
            if i32::abs(diff) > 3 {
                safe = false;
                break;
            }

            match state {
                State::INCREASING => {
                    if diff > 0 {
                        safe = false;
                        break;
                    }
                }
                State::DECREASING => {
                    if diff < 0 {
                        safe = false;
                        break;
                    }
                }
                State::UNKNOWN => {
                    state = if diff > 0 {
                        State::DECREASING
                    } else {
                        State::INCREASING
                    };
                }
            }
        }

        if safe {
            safe_reports += 1;
        }
    }

    safe_reports
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

enum State {
    INCREASING,
    DECREASING,
    UNKNOWN,
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_file;

    #[test]
    fn basic_example() {
        let lines = read_file("test_input.txt").unwrap();
        assert_eq!(count_safe_reports(lines), 2);
    }
}
