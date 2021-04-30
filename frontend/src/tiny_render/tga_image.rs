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

impl Color {
    fn raw(&self) -> Vec<char> {
        vec![self::Color[0], self::Color[1], self::Color[2], self::Color[3]]
    }
}

struct TGAColor {
    color: Color,
    raw: Vec<char>,
    val: i32,
    byte_spp: i32,
}

impl TGAColor {
    fn new() -> TGAColor {
        todo!()
        // TGAColor { color: None, raw: vec![], val: 0, byte_spp: 1 }
    }

    fn color(r: char, g: char, b: char, a: char) -> TGAColor {
        let color = Color::new(r, g, b, a);
        TGAColor { color, raw: vec![], val: 0, byte_spp: 4 }
    }

    fn val_byte_spp(val:i32, byte_spp: i32) -> TGAColor {
        todo!()
    }
}

struct Color {
    r: char,
    g: char,
    b: char,
    a: char,
}

impl Color {
    fn new(r: char, g: char, b: char, a: char) -> Color {
        Color { r, g, b, a }
    }
}
