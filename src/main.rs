extern crate clap;
extern crate bmp;

use clap::{App, Arg};

fn main() {
    let matches = App::new("bmp2arr")
                      .version("1.0")
                      .author("Robby Madruga <robbymadruga@gmail.com>")
                      .about("Converts a monochrome bmp file to a bitmapped C array")
                      .arg(Arg::with_name("LINE_COUNT")
                               .short("c")
                               .takes_value(true)
                               .help("Specifies the number of bytes per row"))
                      .arg(Arg::with_name("FILE")
                               .help("Sets the bmp file to convert")
                               .required(true))
                      .get_matches();

    let bmpfile = matches.value_of("FILE").unwrap();
    let line_wrap = matches.value_of("LINE_COUNT").unwrap_or("16").parse::<usize>().unwrap();
    let bmp = bmp::open(bmpfile).expect("Unrecognized file format");

    let (w, h) = (bmp.get_width(), bmp.get_height());
    println!("#define image_width {}", w);
    println!("#define image_height {}", h);

    let mut v = Vec::new();

    for y in 0..h {
        for x in 0..w {
            let p = bmp.get_pixel(x, y);
            v.push(p.r);
            v.push(p.g);
            v.push(p.b);
        }
    }

    println!("static const char image[] = {{");
    for (i, u) in v.iter().enumerate() {
        let comma = if i == v.len() - 1 {
            ""
        } else {
            ","
        };
        match i % line_wrap {
            0 => print!("\t0x{:02X}{} ", u, comma),
            x if x == line_wrap - 1 => println!("0x{:02X}{}", u, comma),
            _ => print!("0x{:02X}{} ", u, comma),
        }
    }
    println!("}};");
}
