use failure::Error;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

const IMAGE_WIDTH: i32 = 25;
const IMAGE_HEIGHT: i32 = 6;

fn image_check(image_data: &str) -> i32 {
    let freq_map = image_data
        .chars()
        .chunks((IMAGE_WIDTH * IMAGE_HEIGHT) as usize)
        .into_iter()
        .map(|layer| {
            layer.fold(HashMap::<char, i32>::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            })
        })
        .min_by_key(|freq_map| freq_map[&'0'])
        .unwrap();

    freq_map[&'1'] * freq_map[&'2']
}

fn main() -> Result<(), Error> {
    let image_data = fs::read_to_string("data.txt")?;
    let v = image_check(&image_data);
    println!("{}", v);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test() {}
}
