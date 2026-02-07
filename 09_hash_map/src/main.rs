// TODO: print instructions
// TODO: loop to read input until "done"
// TODO: store numbers somewhere
// TODO: find which ones appear > 1 time
// TODO: print the duplicates

use std::{collections::HashMap, io};

fn main() {
    println!("Enter numbers or done to finish : ");
    // let mut v : Vec<i32> = Vec::new();
    let mut v: String = String::new();
    let mut hash: HashMap<u32, u32> = HashMap::new();
    while v.trim() != "done" {
        {
            v.clear();
            io::stdin().read_line(&mut v).unwrap();
            if v.trim() == "done" {
                break;
            }
            let i: u32 = v.trim().parse::<u32>().unwrap();
            // hash.insert(i, +=1);
            *hash.entry(i).or_insert(0) += 1;
        }
    }
    let mut duplicates: Vec<u32> = Vec::new();

    for (number, count) in &hash {
        if *count > 1 {
            duplicates.push(*number);
        }
    }

    if duplicates.is_empty() {
        println!("No duplicates found");
    } else {
        println!("Duplicates found: {:?}", duplicates);

        println!(
            "Duplicates found: {}",
            duplicates
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
