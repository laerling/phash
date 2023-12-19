use std::env::args;
use std::io;
use std::str::from_utf8_unchecked;

use image::error::{ImageError,ImageResult};
use image::io::Reader as ImageReader;


fn phash<'hash>(filename: &String) -> ImageResult<[u8; 16]> {

    // open image file
    let image_reader = ImageReader::open(filename)?;

    // file extensions aren't always correct
    let image_reader = image_reader.with_guessed_format()?;

    // decode image
    let image = image_reader.decode()?;

    // TODO
    return Ok([b'0'; 16]);
}

fn main() {

    // Print perceptive hash for each image file.
    // Skip file name of this executable.
    for filename in args().skip(1) {

        // get perceptive hash (phash) for file
        let file_phash: [u8; 16] = match phash(&filename) {
            Ok(phash) => phash,

            // General IO error
            Err(ImageError::IoError(e)) => match e.kind() {

                // Warn about missing files
                io::ErrorKind::NotFound => {
                    eprintln!("{}: {}", e, filename);
                    continue;
                },

                // All other IO errors are unexpected
                _ => panic!("IO error when opening file {}: {}", filename, e),
            }

            // Unsupported image format
            Err(ImageError::Unsupported(e)) => {
                eprintln!("{}: {}", e, filename);
                continue;
            },

            // All other image errors are unexpected
            Err(e) => panic!("Error decoding file {}: {}", filename, e),

        };

        // print phash
        let file_phash_str: &str = unsafe { from_utf8_unchecked(&file_phash[1..16]) };
        println!("{} {}", file_phash_str, filename);
    }
}
