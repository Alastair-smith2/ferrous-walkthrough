mod files;

use files::Result;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    let mut file = File::open("src/data/content.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);

    println!("The content {:?}", buffer);
    Ok(())
}
