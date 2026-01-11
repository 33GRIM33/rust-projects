use std::error::Error;

use csv;


fn main() {
    // println!("Current dir: {:?}", std::env::current_dir());

    if let Err(e) = read_from_file("./data.csv") {
        println!("Error: {}", e);
    }
}

fn read_from_file(path: &str)-> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}