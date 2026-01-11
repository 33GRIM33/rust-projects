use std::fs;
use std::io;
use std::path::Path;
use zip::result::ZipError;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <zipfile> [output_path]", args[0]);
        return 1;
    }

    let zip_path = Path::new(&args[1]);
    let output_path = args.get(2).map(Path::new);

    if let Err(e) = decompress(zip_path, output_path) {
        eprintln!("Error: {}", e);
        return 1;
    }

    0
}

fn decompress(zip_path: &Path, output_path: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let outpath = if let Some(output_path) = output_path {
            output_path.join(outpath)
        } else {
            outpath
        };

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;
    use zip::write::{FileOptions, ZipWriter};

    #[test]
    fn decompress_test() {
        let dir = tempdir().unwrap();
        let zip_path = dir.path().join("test.zip");
        let extract_path = dir.path().join("extracted");
        fs::create_dir_all(&extract_path).unwrap();

        let file = fs::File::create(&zip_path).unwrap();
        let mut zip = ZipWriter::new(file);

        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        zip.start_file("test.txt", options).unwrap();
        zip.write_all(b"Hello, world!").unwrap();
        zip.finish().unwrap();

        let result = decompress(&zip_path, Some(&extract_path));
        assert!(result.is_ok());

        let extracted_file_path = extract_path.join("test.txt");
        assert!(extracted_file_path.exists());

        let content = fs::read_to_string(extracted_file_path).unwrap();
        assert_eq!(content, "Hello, world!");
    }
}
