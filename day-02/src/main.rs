use day_02::count_safe_reports_with_reverification;
use utils::read_file;

fn main() {
    let lines = read_file("input.txt").unwrap();
    println!("{}", count_safe_reports_with_reverification(lines));
}
