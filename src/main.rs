mod durable;

use durable::{DurableFile, Result};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    let file = File::create("src/data/example.txt")?;
    let mut durable_file = DurableFile::from(file);
    durable_file.write(b"Hello, world!")?;
    durable_file.close()?;
    Ok(())
}
