use std::env::args;
use std::io::ErrorKind::NotFound;
use std::str::from_utf8_unchecked;

fn phash<'hash>(filename: &String) -> std::io::Result<[u8; 16]> {
    // TODO
    return Ok([b'0'; 16]);
}

fn main() {

    // Print perceptive hash for each image file.
    // Skip file name of this executable.
    for filename in args().skip(1) {

        // get perceptive hash (phash) for file
        let file_phash: [u8; 16] = match phash(&filename) {
            Ok(hash) => hash,

            // Ignore files that are not images
            // TODO

            // Warn about files that could not be found
            Err(e) if e.kind() == NotFound => {
                eprintln!("File not found: {}", filename);
                continue;
            },

            // All other errors are unexpected, so panic
            Err(e) => panic!("Failed creating perceptive hash of file {}: {}",
                             filename, e),
        };

        // print phash
        let file_phash_str: &str = unsafe { from_utf8_unchecked(&file_phash[1..16]) };
        println!("{} {}", file_phash_str, filename);
    }
}
