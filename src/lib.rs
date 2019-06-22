use std::fs::{self, File};
use std::io::Write;
use std::panic;

const PATH: &str = "lines.txt";

#[allow(dead_code)]
fn set_up() {
    let mut output = File::create(PATH).expect("Could not create a file.");
    write!(output, "Rust\n💖\nFun").expect("Could not write to the file.");
}

#[allow(dead_code)]
fn tear_down() {
    fs::remove_file(PATH).expect("Could not remove the file.");
}

#[allow(dead_code)]
fn run_test<T>(test: T)
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    set_up();

    let result = panic::catch_unwind(test);

    tear_down();

    assert!(result.is_ok())
}

#[cfg(test)]
mod file_reading_tests {
    use super::run_test;
    use super::PATH;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn read_file() {
        run_test(|| {
            let input = File::open(PATH).expect("Could not open the file.");
            let buffered = BufReader::new(input);
            let mut lines = vec![];
            for line in buffered.lines() {
                lines.push(line.expect("Expected a line."));
            }

            assert_eq!(vec!["Rust", "💖", "Fun",], lines);
        })
    }
}
