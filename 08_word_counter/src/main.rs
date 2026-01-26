    // TODo: read the file done
    // TODO: count words (split by spaces)
    // TODO: print result
    
    // DONE: get filename from command line 
    use std::env;
    use std::fs;

fn main() {

    let args:Vec<String> = env::args().collect();
    // dbg!(args);
    // let contents = fs::read(&args[1]).unwrap();
    let contents = fs::read_to_string(&args[1]).unwrap();
    // Mistake:
    // I tried to access args[1] directly, which attempts to MOVE a String
    // out of a Vec<String>. Rust does not allow moving ownership out of a
    // vector by indexing, especially for heap-allocated types like String
    // which do not implement the Copy trait.
    //
    // Fix:
    // Borrow the value instead of moving it (use &args[1]) or iterate over
    // the vector. Vectors allow borrowing, not ownership transfer via indexing.
    //
    // dbg!(contents);
    // let a = "Hello world! This is a test.";
    let mut v: Vec<String> = Vec::new();
    let mut w: String = String::new();
    
    for i in contents.chars() {
        if  i==' '{
            if !w.is_empty(){
                v.push(w);
                w = String::new();
            }
            // have to create a new String here, instead of w ="" because
            // w is a String type, not a &str type
            // we cant push &str into Vec<String>
            continue;
        }
        w.push(i);
        // println!(" {} ",i);
    }
    // v.push(w);
    println!("word count: {}", v.len());



    //one could also do this with split_whitespace
    //this is rusty method 
    let word_count = contents.split_whitespace().count();
    println!("word count: {}", word_count);

}
