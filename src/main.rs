mod files;

use files::Result;
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
        println!("The content {:?} on line", line);
        number_lines += 1;
    }
    println!("The number of lines {:?}", number_lines);
    Ok(())
}
