// use std::io::Read;
// // use error_chain::error_chain;

// // error_chain! {
// //     foreign_links {
// //         Io(std::io::Error);
//     //     HttpRequest(reqwest::Error);
//     // }
// // }



// fn main() -> Result<()> {
//     let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;
//     println!("{}", body);
//     Ok(())
// }

use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    println!("{}", body);
    Ok(())
}
