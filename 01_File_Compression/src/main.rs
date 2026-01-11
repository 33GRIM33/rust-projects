extern crate flate2;

use flate2::Compression;
use flate2::write::GzEncoder;

use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    if args().len() !=3 {
        eprint!("Usage: {} <infile> <outfile>", args().nth(0).unwrap());
        return;
    }
    let mut input = BufReader::new(File::open(&args().nth(1).unwrap()).unwrap());
    let output = File::create(&args().nth(2).unwrap()).unwrap();
    
    let mut encoder = GzEncoder::new(output, Compression::default());
    
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    
    let duration = start.elapsed();
    println!("Compressed in {} ms", duration.as_millis());
    println!("Source len : {:?}",input.get_ref().metadata().unwrap().len());
    println!("Output len : {:?}",output.metadata().unwrap().len());
}
