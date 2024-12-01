use std::str::FromStr;

pub fn total_distance_and_similarity_score(input: Vec<String>) -> (i32, i32) {
    let (mut left, mut right) = convert(input);
    left.sort();
    right.sort();
    let mut distance = 0;
    let mut score = 0;

    for it in left.iter().zip(right.iter()) {
        let (a, b) = it;
        distance += i32::abs(a - b);
    }

    for l in left.iter() {
        score += right.iter().filter(|r| **r == *l).sum::<i32>();
    }

    (distance, score)
}

fn convert(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input {
        let mut split = line.split("   ");
        left.push(i32::from_str(split.next().unwrap()).unwrap());
        right.push(i32::from_str(split.next().unwrap()).unwrap());
    }
    
    (left, right)
}



#[cfg(test)]
mod tests {
    use super::*;
    use utils::read_file;

    #[test]
    fn basic_example() {
        let input = read_file("test_input.txt").unwrap();
        assert_eq!(total_distance_and_similarity_score(input), (11, 31));
    }
}
