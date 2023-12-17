//use image::io::Reader as ImageReader;

// FIXME write hash to byte array and move-return reference to it (in Result of course).
// FIXME probably another (more Image crate specific) Result type makes sense here.
fn phash(filename: &String) -> std::io::Result<String> {
    //ImageReader::open(filename)?;
    return Ok(String::from("[placeholder]"));
}

fn main() {
    for filename in std::env::args().skip(1) {
        let err_msg = format!("Failed creating perceptive hash of file {}", filename);
        let file_phash = phash(&filename).expect(err_msg.as_str());
        println!("{} {}", file_phash, filename);
    }
}
