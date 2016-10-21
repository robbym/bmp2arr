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

fn compress_image(image: &Vec<u8>) -> Vec<u8> {
    let mut byte_count = [0u8; 256];
    let mut max_idx = 0;
    let mut max_val = 0;
    for b in image {
        let idx = *b as usize;
        if byte_count[idx] < 255 {
            byte_count[idx] += 1;
        }

        if byte_count[idx] > max_val {
            max_idx = idx;
            max_val = byte_count[idx];
        }
    }

    let mut v: Vec<u8> = Vec::new();

    let mut idx: usize = 0;
    let mut count: u16 = 0;
    loop {
        if image[idx] == max_idx as u8 {
            count += 1;
        } else if count > 4 {
            v.push(0x18);
            v.push(0xE7);
            v.push(max_idx as u8);
            v.push(count as u8);
            v.push((count >> 8) as u8);
            v.push(image[idx]);
            count = 0;
        } else {
            while count > 0 {
                v.push(max_idx as u8);
                count -= 1;
            }
            v.push(image[idx]);
        }

        if idx < image.len()-1 {
            idx += 1;
        } else {
            break;
        }
    }
    
    v
}

fn print_image(v: &Vec<u8>, w: u32, h: u32, line_wrap: usize) {
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
    let output_size = (v.len()*(bitcount as usize)/8) as usize;
    r.reserve(output_size);

    for (i, v) in v.iter().enumerate() {
        let bit_idx = (i as u32)*bitcount;
        let bits_left = 8 - (bit_idx%8);
        let value = v >> (8-bitcount);

        while r.len() < ((1 + bit_idx/8) as usize) { r.push(0); }

        if bits_left < bitcount {
            r.push(0);

            let overflow = bitcount-bits_left;
            r[(bit_idx/8) as usize] |= value >> overflow;
            r[(bit_idx/8 + 1) as usize] |= value << (8-overflow);
        } else {
            r[(bit_idx/8) as usize] |= value << (8-bitcount-(bit_idx%8));
        }
    }

    let compressed = compress_image(&r);
    println!("Normal: {}, Compressed: {}", r.len(), compressed.len());
    print_image(&compressed, w, h, line_wrap);
}
