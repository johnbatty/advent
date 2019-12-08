 use failure::Error;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

const IMAGE_WIDTH: i32 = 25;
const IMAGE_HEIGHT: i32 = 6;
const LAYER_SIZE: i32 = IMAGE_WIDTH * IMAGE_HEIGHT;

const TRANSPARENT_COLOR: u8 = '2' as u8;

fn pixel_color(x: i32, y: i32, image_data: &[u8]) -> char {
    let num_layers = image_data.len() as i32 / LAYER_SIZE;
    for layer in 0..num_layers {
        let index = (layer * LAYER_SIZE) + (y * IMAGE_WIDTH) + x;
        let pixel = image_data[index as usize];
        if pixel != TRANSPARENT_COLOR {
            return pixel as char;
        }
    }
    return '0';
}

fn render_image_output(image_data: &[u8]) -> String {
    let mut s = String::new();
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            s += match pixel_color(x, y, image_data) {
                '0' => " ",
                '1' => "*",
                _ => "?",
            }
        }
        s += "\n";
    }
    s
}


fn main() -> Result<(), Error> {
    let image_data = fs::read_to_string("data.txt")?;
    let s = render_image_output(&image_data.as_bytes());
    println!("{}", &s);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test() {}
}
