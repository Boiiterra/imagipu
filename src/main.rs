use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    let filepath = "picture.png";
    let xfactor: u32 = 4;
    let yfactor: u32 = 4;
    assert!(xfactor >= 1);
    assert!(yfactor >= 1);

    let img = match image::open(filepath) {
        Ok(img) => img,
        Err(e) => {
            println!("Something wrong happened: {}", e);
            return;
        }
    };

    let (width, height): (u32, u32) = img.dimensions();
    println!("[INFO]: Reading image with size: {}x{}.", width, height);
    let (imgx, imgy): (u32, u32) = (width / xfactor, height / yfactor);
    println!("[INFO]: Creating image with size: {}x{}.", imgx, imgy);

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    println!("STATUS -> Generating image.");
    for x in (0..width).step_by(xfactor as usize) {
        if x + 1 == width || x / xfactor >= imgx {
            break; // Some errors may happen
        }
        for y in (0..height).step_by(yfactor as usize) {
            if y + 1 == height || y / yfactor >= imgy {
                break; // Some errors may happen
            }

            let mut r: u16 = 0;
            let mut g: u16 = 0;
            let mut b: u16 = 0;
            let mut a: u16 = 0;
            let mut m: u16 = 0;

            for nx in 0..xfactor {
                if x + nx >= width {
                    break;
                }
                for ny in 0..yfactor {
                    if y + ny >= height {
                        break;
                    }

                    let p: Rgba<u8> = img.get_pixel(x + nx, y + ny);

                    r += p[0] as u16;
                    g += p[1] as u16;
                    b += p[2] as u16;
                    a += p[3] as u16;
                    m += 1;
                }
            }

            let r: u8 = (r / m) as u8;
            let g: u8 = (g / m) as u8;
            let b: u8 = (b / m) as u8;
            let a: u8 = (a / m) as u8;

            let pixel = Rgba([r, g, b, a]);
            imgbuf.put_pixel(x / xfactor, y / yfactor, pixel);
        }
    }
    println!("STATUS -> image is generated.");

    println!("STATUS -> writing image to file.");
    imgbuf.save("smol_pic.png").unwrap();
    println!("STATUS -> image is written.");
}
