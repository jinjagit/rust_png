use png;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let path = Path::new(r"../image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 1024, 1024);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let mut data: [u8; 4194304] = [0; 4194304]; // Initialize array of zeros, to become the RGBA sequence. 4 values per pixel.
    
    for i in 0..4194303 {
        if i <= 2097151 && i % 4 == 0 { // top half, red channel
            data[i] = 255 as u8;
        } else if i > 2097151 && i % 4 == 1 { // bottom half, green channel
            data[i] = 255 as u8;
        }

        if i % 4 == 3 {
            data[i] = 255 as u8; // Alpha = 255
        }
    }
    writer.write_image_data(&data).unwrap(); // Save
}
