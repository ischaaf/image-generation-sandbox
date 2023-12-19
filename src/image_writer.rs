use std::fs;

use png_encode_mini;

pub fn write(filename: &str, data: Vec<u32>, width: u32, height: u32) {
    let mut file = fs::File::create(filename).unwrap();
    png_encode_mini::write_rgba_from_u32(&mut file, data.as_slice(), width, height).unwrap();
}
