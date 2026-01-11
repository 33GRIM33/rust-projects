use std::fs;
use std::io;
use std::path::Path;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    // Collect command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Expect: program <zip_file>
    if args.len() != 2 {
        eprintln!("Usage: {} <zipfile>", args[0]);
        return 1;
    }

    let zip_path = Path::new(&args[1]);

    // Open the zip file
    let file = match fs::File::open(zip_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open zip file: {}", e);
            return 1;
        }
    };

    // Create zip archive reader
    let mut archive = match zip::ZipArchive::new(file) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Invalid zip archive: {}", e);
            return 1;
        }
    };

    // Iterate through files in the zip
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap();

        // Prevent zip-slip (path traversal attack)
        let outpath = match zip_file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Print optional comments
        let comment = zip_file.comment();
        if !comment.is_empty() {
            println!(
                "Extracting {} ({} bytes): {}",
                outpath.display(),
                zip_file.size(),
                comment
            );
        }

        // Directory entry
        if zip_file.name().ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } 
        // File entry
        else {
            // Ensure parent directories exist
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut zip_file, &mut outfile).unwrap();

            // Preserve unix permissions (if present)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = zip_file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }
    }

    0
}
