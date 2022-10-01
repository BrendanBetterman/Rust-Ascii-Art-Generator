use image::{GenericImageView, Pixel};

const fn get_ascii_pixel(intensity: u8) -> char {
    [' ', '.', ',', '-', '~', '+', '=', '@'][intensity as usize / 32]
}

fn get_image(dir: &str, scale: u32) {
    let img = image::open(dir).unwrap();
    println!("{:?}", img.dimensions());
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let intensity = if pix[3] == 0 {
                    0
                } else {
                    pix.to_luma()[0]
                };
                print!("{}", get_ascii_pixel(intensity));
            }
        }
        if y % (scale * 2) == 0 {
            println!("");
        }
    }
}

fn main() {
    get_image("assets/pug.png", 4);
}
