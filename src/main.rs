use std::env::args;
use std::io;
use std::str::from_utf8_unchecked;

use image::DynamicImage;
use image::error::ImageError;
use image::io::Reader as ImageReader;


fn phash<'hash>(filename: &String) -> io::Result<[u8; 16]> {

    // open image file
    let image_reader = ImageReader::open(filename)?;

    // file extensions aren't always correct
    let image_reader = image_reader.with_guessed_format()?;

    // decode image
    let image: DynamicImage = match image_reader.decode() {
        Ok(i) => i,

        // All IO errors are handled by the callers
        Err(ImageError::IoError(e)) => return Err(e),

        // Return Err so that main can handle it
        // FIXME wrap ImageError, so that main can unpack and inspect it
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };
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

            // Warn about files that could not be found
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                eprintln!("File not found: {}", filename);
                continue;
            },

            // Ignore files that are not images
            Err(e) => match e.get_ref() {

                // print warning about non-decodable files
                //ImageError::Unsupported(_)
                Some(e_image) => { 
                    eprintln!("{}: {}", e_image, filename);
                    continue;
                },

                // None means any other error.
                // All other errors are unexpected, so panic.
                _ => panic!("Failed creating perceptive hash of file {}: {}",
                            filename, e),
            },
        };

        // print phash
        let file_phash_str: &str = unsafe { from_utf8_unchecked(&file_phash[1..16]) };
        println!("{} {}", file_phash_str, filename);
    }
}
