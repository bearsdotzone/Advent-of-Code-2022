use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut buffer = String::from("");
    let _n = file.read_to_string(&mut buffer)?;

    let mut max = 0;

    let elves = buffer.split("\n\n");

    let mut elf_iter = elves.into_iter();

    let mut whole_elf = elf_iter.next();

    while !whole_elf.is_none() {
        let elf_nugs = whole_elf.unwrap().split("\n");

        let out = elf_nugs.fold(0, |accum, item| {
            let parsed_cal = i32::from_str(item);
            match parsed_cal {
                Ok(parsed_cal) => accum + parsed_cal,
                Err(_) => accum,
            }
        });

        if out >= max {
            max = out;
        }

        whole_elf = elf_iter.next();
    }

    println!("{}", max);

    Ok(())
}
