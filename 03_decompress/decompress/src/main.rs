use std::fs;
use std::io;



fn main() {
    std::process::exit(real_main());
}

fn real_main()->i32{
    let args:Vec<_> = std::env::args().collect;

    if args.len()!=3{
        eprintln!("Usage: {} <infile> ", args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&args[1]);

    let file = fs::file::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len(){
        let mut file = archive.by_index(i).unwrap();
        
    }

}
