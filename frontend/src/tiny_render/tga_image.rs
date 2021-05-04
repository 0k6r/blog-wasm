use std::io::{BufReader, BufWriter};

struct TGAHeader {
    id_length: char,
    color_map_type: char,
    data_type_code: char,
    color_map_origin: u16,
    color_map_length: u16,
    color_map_depth: char,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: char,
    image_descriptor: char,
}

enum Color {
    RGBA(char, char, char, char)
}

impl Color {
    fn new(r: char, g: char, b: char, a: char) -> Self {
        Self { r, g, b, a }
    }

    fn raw(&self) -> Vec<char> {
        vec![self::Color[0], self::Color[1], self::Color[2], self::Color[3]]
    }
}

#[derive(Clone)]
struct TGAColor {
    color: Option<Color>,
    raw: Vec<char>,
    val: i32,
    byte_spp: i32,
}

impl TGAColor {
    fn new() -> Self {
        TGAColor { color: None, raw: vec![], val: 0, byte_spp: 1 }
    }

    fn color(r: char, g: char, b: char, a: char) -> Self {
        let color = Some(Color::RGBA(r, g, b, a));
        Self { color, raw: vec![], val: 0, byte_spp: 4 }
    }

    fn val_byte_spp(p: Vec<char>, byte_spp: i32) -> Self {
        todo!()
    }

    fn point(&p: char, byte_spp: i32) -> Self {
        let mut raw: Vec<char> = Vec::new();
        for byte in 0..byte_spp {
            raw.push(p[byte])
        }
        Self { color: None, raw, val: 0, byte_spp }
    }
}

enum Format {
    GRAYSCALE = 1,
    RGB = 3,
    RGBA = 4,
}

trait Image {
    fn load_rle_data(input: BufReader<()>) -> Result<(), ()>;
    fn unload_rle_data(output: BufWriter<()>) -> Result<(), ()>;
    fn new() -> Self;
    fn new_resolution(w: i32, h: i32, bpp: i32) -> Self;
    fn clone(from: &dyn Image) -> Self;
    fn read_tga_file(filename: &char) -> bool;
    fn write_tha_file(filename: &char, rle: bool) -> bool;
    fn flip_horizontally() -> bool;
    fn flip_vertically() -> bool;
    fn scale(w: i32, h: i32) -> bool;
    fn get(x: i32, y: i32) -> Self;
    fn set(x: i32, y: i32, img: &Self) -> bool;
    fn drop(&mut self);
    fn get_width() -> i32;
    fn get_height() -> i32;
    fn get_byte_spp() -> i32;
    fn buffer() -> &char;
    fn clear();
}

struct TGAImage {
    data: char,
    width: i32,
    height: i32,
    byte_spp: i32,
}

impl Image for TGAImage {
    fn load_rle_data(input: BufReader<()>) -> Result<(), ()> {
        todo!()
    }

    fn unload_rle_data(output: BufWriter<()>) -> Result<(), ()> {
        todo!()
    }

    fn new() -> Self {
        todo!()
    }

    fn new_resolution(w: i32, h: i32, bpp: i32) -> Self {
        todo!()
    }

    fn clone(from: &dyn Image) -> Self {
        todo!()
    }

    fn read_tga_file(filename: &char) -> bool {
        todo!()
    }

    fn write_tha_file(filename: &char, rle: bool) -> bool {
        todo!()
    }

    fn flip_horizontally() -> bool {
        todo!()
    }

    fn flip_vertically() -> bool {
        todo!()
    }

    fn scale(w: i32, h: i32) -> bool {
        todo!()
    }

    fn get(x: i32, y: i32) -> Self {
        todo!()
    }

    fn set(x: i32, y: i32, img: &Self) -> bool {
        todo!()
    }

    fn drop(&mut self) {
        todo!()
    }

    fn get_width() -> i32 {
        todo!()
    }

    fn get_height() -> i32 {
        todo!()
    }

    fn get_byte_spp() -> i32 {
        todo!()
    }

    fn buffer() -> &char {
        todo!()
    }

    fn clear() {
        todo!()
    }
}
