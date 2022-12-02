use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut buffer = String::from("");
    let _n = file.read_to_string(&mut buffer)?;

    // let mut max = 0;

    let mut treemap = BTreeMap::<i32, i32>::new();

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

        if treemap.len() < 3 {
            treemap.insert(out, out);
        } else if out > *treemap.first_entry().unwrap().key() {
            treemap.pop_first();
            treemap.insert(out, out);
        }

        whole_elf = elf_iter.next();
    }

    let mut sum = 0;

    for a in treemap.clone() {
        sum += a.0;
    }

    println!("{:?} {}", treemap.last_key_value().unwrap().0, sum);

    Ok(())
}
