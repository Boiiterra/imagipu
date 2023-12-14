use std::fmt::Display;

use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};

enum LogLevel {
    Note,
    Info,
    Warning,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Note    => write!(f, "- [NOTE]:"),
            LogLevel::Info    => write!(f, "- [INFO]:"),
            LogLevel::Warning => write!(f, "- [WARNING]:"),
            LogLevel::Error   => write!(f, "- [ERROR]:"),
        }
    }
}

const N: LogLevel = LogLevel::Note;
const I: LogLevel = LogLevel::Info;
const W: LogLevel = LogLevel::Warning;
const E: LogLevel = LogLevel::Error;

struct Image {
    src: DynamicImage,
    width: u32,
    height: u32,
}

impl Image {
    fn from_file(filepath: &str) -> Self {
        println!("{N} Reading image form {}.", filepath);
        let src = match image::open(filepath) {
            Ok(img) => img,
            Err(err) => {
                panic!("{E} Wrong things happened:\n{}", err);
            }
        };

        let (width, height) = src.dimensions();
        println!("{N} Reading image with size: {}x{}.", width, height);

        Self {
            src,
            width,
            height,
        }
    }
}

struct Buffer {
    buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    width: u32,
    height: u32,
}

impl Buffer {
    fn new(width: u32, height: u32) -> Self {
        Self {
            buf: ImageBuffer::new(width, height),
            width,
            height,
        }
    }
}

fn smol(img: Image, xfactor: u32, yfactor: u32, buffer: &mut Buffer) -> bool {
    for x in 0..buffer.width {
        for y in 0..buffer.height {
            let mut r: u16 = 0;
            let mut g: u16 = 0;
            let mut b: u16 = 0;
            let mut a: u16 = 0;
            let mut m: u16 = 0;

            for nx in 0..xfactor {
                if x + nx >= img.width {
                    break;
                }
                for ny in 0..yfactor {
                    if y + ny >= img.height {
                        break;
                    }

                    let p: Rgba<u8> = img.src.get_pixel(x * xfactor + nx, y * yfactor + ny);

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
            buffer.buf.put_pixel(x, y, pixel);
        }
    }
    return true;
}

fn main() {
    let filepath = "picture.png";
    let xfactor: u32 = 4;
    let yfactor: u32 = 4;
    assert!(xfactor >= 1);
    assert!(yfactor >= 1);

    let img: Image = Image::from_file(filepath);
    let (imgx, imgy): (u32, u32) = (img.width / xfactor, img.height / yfactor);
    let mut buf: Buffer = Buffer::new(imgx, imgy);

    println!("{N} Creating image with size: {}x{}.", imgx, imgy);

    println!("{I} STATUS -> Generating image.");
    smol(img, xfactor, yfactor, &mut buf);
    println!("{I} STATUS -> image is generated.");

    println!("{I} STATUS -> writing image to file.");

    println!("{W} We may fail here.");
    buf.buf.save("smol_pic.png").unwrap();
    println!("{I} STATUS -> image is written.");

}
