use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut buffer = String::from("");
    file.read_to_string(&mut buffer)?;
    let rounds = buffer.split("\n");

    let mut round_one_score = 0;
    let mut round_two_score = 0;

    for i in rounds {
        // convert A B C X Y Z to 1 2 and 3.
        let play1: i16 = (i.bytes().nth(0).unwrap() - 64).into();
        let play2: i16 = (i.bytes().nth(2).unwrap() - 87).into();

        //play points
        round_one_score += play2;

        // result points
        if play2 - play1 == 1 || play2 - play1 == -2 {
            // win
            round_one_score += 6;
        } else if play1 - play2 == 0 {
            // tie
            round_one_score += 3;
        } else {
            // loss
            round_one_score += 0;
        }

        //result points
        round_two_score += (play2 - 1) * 3;

        //play points
        if play2 == 1 {
            // loss
            if play1 - 1 == 0 {
                round_two_score += 3;
            } else {
                round_two_score += play1 - 1;
            }
        } else if play2 == 2 {
            // tie
            round_two_score += play1;
        } else {
            //win
            if play1 + 1 == 4 {
                round_two_score += 1;
            } else {
                round_two_score += play1 + 1;
            }
        }
    }

    println!("round one: {}", round_one_score);
    println!("round two: {}", round_two_score);

    Ok(())
}
