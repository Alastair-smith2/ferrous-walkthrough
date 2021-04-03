mod files;

use files::{parse_url, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<()> {
    let file = File::open("src/data/content.txt")?;
    let mut reader = BufReader::new(file);
    let desired_content = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|s| !s.is_empty());

    let mut number_lines = 0;
    for line in desired_content {
        number_lines += 1;
        if let Some(option_url) = parse_url(line) {
            println!("The url {:?}", option_url);
        }
    }
    println!("The number of lines {:?}", number_lines);
    Ok(())
}
