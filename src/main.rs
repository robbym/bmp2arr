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

    let v = (0..w * h)
        .map(|i| bmp.get_pixel(i % w, i / w))
        .fold(Vec::new(), |mut v, p| {
            v.push(p.r);
            v.push(p.g);
            v.push(p.b);
            v
        });

    let mut ps = Vec::<u8>::new();

    for i in 0..w * h * 3 {
        if ps.len() > i as usize {
            ps[(i / 4) as usize] |= (v[i as usize] >> 6) << (2 * (i % 4));
        } else {
            ps.push(v[i as usize] >> 6);
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
