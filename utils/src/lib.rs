use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_file(file_name: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file)
        .lines()
        .map(|l| l.expect("Failed to parse the line"))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_check() {
        let lines = read_file("test_input.txt").unwrap();
        assert_eq!(lines[0], String::from("1 2 3 4"));
        assert_eq!(lines[1], String::from("5 6 7 8"));
    }

    #[test]
    fn file_doesnt_exists() {
        let err = read_file("bad_path");
        assert!(err.is_err());
    }
}
