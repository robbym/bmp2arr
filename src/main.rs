extern crate clap;
extern crate bmp;
use clap::{App, Arg, ArgMatches};

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("bmp2arr")
        .version("1.0")
        .author("Robby Madruga <robbymadruga@gmail.com>")
        .about("Converts a bmp image to a bitmapped C array")
        .arg(Arg::with_name("BIT_COUNT")
            .short("b")
            .takes_value(true)
            .help("Specifies how many bits per color"))
        .arg(Arg::with_name("LINE_COUNT")
            .short("c")
            .takes_value(true)
            .help("Specifies the number of bytes per row"))
        .arg(Arg::with_name("FILE")
            .help("Sets the bmp file to convert")
            .required(true))
        .get_matches()
}

fn print_image(v: Vec<u8>, w: u32, h: u32, line_wrap: usize) {
    println!("#define image_width {}", w);
    println!("#define image_height {}", h);

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
    println!("\n}};");
}

fn main() {
    let matches = parse_args();

    let bmpfile = matches.value_of("FILE").unwrap();
    let bitcount = matches.value_of("BIT_COUNT").unwrap_or("8").parse::<u32>().unwrap();
    let line_wrap = matches.value_of("LINE_COUNT").unwrap_or("16").parse::<usize>().unwrap();
    let bmp = bmp::open(bmpfile).expect("Unrecognized file format");

    if bitcount > 8 {
        println!("Bit counts above 8 are currently not supported");
        return;
    }

    let (w, h) = (bmp.get_width(), bmp.get_height());

    let v: Vec<u8> = bmp.coordinates()
        .map(|(x, y)| bmp.get_pixel(x, y))
        .flat_map(|p| vec![p.r, p.g, p.b])
        .collect();

    let mut r: Vec<u8> = Vec::new();
    r.reserve((w * h * 3) as usize);

    for (i, v) in v.iter().enumerate() {
        let bit_idx = (i as u32)*bitcount/8;
        let bit_sht = 
    }

    print_image(v, w, h, line_wrap);
}
