use std::str::FromStr;
use std::io::{LineWriter,BufReader,BufRead};

mod processor;

fn main() {
    let line = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let mut reader = BufReader::new(std::io::stdin());
    let mut writer = LineWriter::new(std::io::stdout());

    let program = line
        .split(',')
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .collect();

    let mut processor = processor::Processor::initialize(program, &mut reader, &mut writer);
    processor.run();
}
