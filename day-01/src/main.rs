use day_01::total_distance_and_similarity_score;
use utils::read_file;

fn main() {
    let input = read_file("input.txt").unwrap();
    let (distance, score) = total_distance_and_similarity_score(input);
    println!("{}", distance);
    println!("{}", score);
}
