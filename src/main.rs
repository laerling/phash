use std::env::args;
use std::io;
use std::str::from_utf8_unchecked;

use image::error::{ImageError,ImageResult};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;


fn phash<'hash>(filename: &String) -> ImageResult<[u8; 16]> {

    // open image file
    let image_reader = ImageReader::open(filename)?;

    // file extensions aren't always correct
    let image_reader = image_reader.with_guessed_format()?;

    // decode image
    let image = image_reader.decode()?;

    // convert to grayscale, since we're not interested in color shifts
    let image = image.grayscale();

    // resize
    // TODO: Which filter gives best result?
    //       Triangle (fastest), CatmullRom, Gaussian, Lanczos3
    let image = image.resize_exact(4, 4, FilterType::Lanczos3);

    // get pixels
    let image_bytes: Vec<u8> = image.into_bytes();
    assert_eq!(image_bytes.len(), 16, "(image: {})", filename);

    // downsample pixels, convert to hex, write to phash buffer
    let mut phash: [u8; 16] = [b'0'; 16];
    for i in 0..16 {
        let ds: u8 = image_bytes[i] / 16;
        let err_msg = "Somehow dividing a byte by 16 turned out bigger than 0xf";
        phash[i] = "0123456789abcdef".bytes().nth(usize::from(ds)).expect(err_msg);
    }

    Ok(phash)
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
        // We can use unsafe from_utf8_unchecked because we know that it's only [0-9a-f]
        let file_phash_str: &str = unsafe { from_utf8_unchecked(&file_phash[0..16]) };
        println!("{} {}", file_phash_str, filename);
    }
}
