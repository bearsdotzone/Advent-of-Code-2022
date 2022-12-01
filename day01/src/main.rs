use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = file.read(&mut buffer[..])?;
    print!("{}", n);
    Ok(())
}
