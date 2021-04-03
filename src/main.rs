use std::fs::File;
fn main() {
    let f = File::open("src/data/content.txt");
    match f {
        Ok(file) => {
            println!("The file content {:?}", file);
        }
        Err(err) => {
            println!("The error {:?}", err)
        }
    }
}
