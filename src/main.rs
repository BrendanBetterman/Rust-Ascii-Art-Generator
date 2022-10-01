use image::{GenericImageView, Pixel};

const fn get_ascii_pixel(intensity: u8) -> char {
    [' ', '.', ',', '-', '~', '+', '=', '@'][intensity as usize / 32]
}

fn get_image(dir: &str, columns: u32) {
    let img = image::open(dir).unwrap();
    println!("{:?}", img.dimensions());
    let (width, height) = img.dimensions();
    let x_scale = width as f32 / columns.min(width) as f32;
    let y_scale = x_scale * 2.;
    for y in 0.. {
        let yy = (y as f32 * y_scale) as u32;
        if yy >= height {
            break;
        }

        for x in 0.. {
            let xx = (x as f32 * x_scale) as u32;
            if xx >= width {
                break;
            }

            let pix = img.get_pixel(xx, yy);
            let intensity = if pix[3] == 0 {
                0
            } else {
                pix.to_luma()[0]
            };
            print!("{}", get_ascii_pixel(intensity));
        }

        println!();
    }
}

fn main() {
    get_image("assets/pug.png", 157);
}
