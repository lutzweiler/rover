use bevy::prelude::Color;
use std::path::Path;

pub fn file_exists(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err("File not found".to_string())
    }
}

pub fn str_to_color(color: &str) -> Result<Color, String> {
    let first = color.chars().nth(0).ok_or("parsing error".to_string())?;
    let mut offset = 0;
    if first == '#' {
        offset = 1;
    }
    let r = hex_to_int(&color[offset + 0..offset + 2]).ok_or("parsing error".to_string())? as u8;
    let g = hex_to_int(&color[offset + 2..offset + 4]).ok_or("parsing error".to_string())? as u8;
    let b = hex_to_int(&color[offset + 4..offset + 6]).ok_or("parsing error".to_string())? as u8;
    Ok(Color::rgb_u8(r, g, b))
}

pub fn hex_char_val(c: char) -> Option<u8> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'a' | 'A' => Some(10),
        'b' | 'B' => Some(11),
        'c' | 'C' => Some(12),
        'd' | 'D' => Some(13),
        'e' | 'E' => Some(14),
        'f' | 'F' => Some(15),
        _ => None,
    }
}

pub fn hex_to_int(hex: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for c in hex.chars() {
        sum *= 16;
        sum += hex_char_val(c)? as u32;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_int() {
        assert_eq!(super::hex_to_int("FF").unwrap(), 255);
        assert_eq!(super::hex_to_int("1000").unwrap(), 1 << 12);
        assert_eq!(super::hex_to_int("0").unwrap(), 0);
        assert_eq!(super::hex_to_int("00ab").unwrap(), super::hex_to_int("AB").unwrap());
    }

    #[test]
    fn hex_to_color() {
        assert_eq!(str_to_color("ff00ff").unwrap(), Color::rgb(1., 0., 1.));
        assert_eq!(str_to_color("#0000ff").unwrap(), Color::rgb(0., 0., 1.));
    }
}
