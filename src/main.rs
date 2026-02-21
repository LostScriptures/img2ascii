use image::{GenericImageView, ImageReader};
use image::imageops::Gaussian;
use std::env;
use std::process::exit;


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut scalling: f64 = 0.3;
    #[allow(unused_assignments)]
    let mut img_path = "";

    if args.len() == 3 {
        scalling = args[1].parse().unwrap();
        img_path = &args[2];
    
    } else if args.len() == 2 {
        img_path = &args[1];
    
    } else {
        println!("Usage: img2ascii <scalling> [path to image]\nAlso works without scalling (defaults to {})", scalling);
        exit(1);
    }

    let ascii = vec!["#", "$", "&", "%", "=", "+", "-", "."];
    let gray_img = ImageReader::open(img_path)
        .unwrap()
        .decode()
        .unwrap();
    
    let dims = gray_img.dimensions();
    let width = (dims.0 as f64 * scalling) as u32;
    let height = (dims.1 as f64 * scalling) as u32;

    let img_vec = gray_img.resize(width, height, Gaussian)
        .into_luma8();
    
    for row in img_vec.enumerate_rows() {
        let mut new_row = String::new();

        for pixel in row.1 {
            let brightness = pixel.2[0];

            new_row.push_str(match brightness {
                225.. => ascii[0],
                193..=224 => ascii[1],
                161..=192 => ascii[2],
                129..=160 => ascii[3],
                97..=128 => ascii[4],
                65..=96 => ascii[5],
                33..=64 => ascii[6],
                0..=32 => ascii[7],
            });
            new_row.push_str(" ");
        }
        println!("{}", new_row)
    }
}