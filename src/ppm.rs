/*
Render a Image in one of the Netpbm formats.
- Portable PixMap format    (PPM)
- Portable GrayMap format   (PGM)
- Portable BitMap format    (PBM)

P1, P2, P3
// comments

*/

use std::{fs::File, io::{Error, Write}, path::Path};

pub fn render_ppm_image_ascii(
    output_file_path: &Path,
    height: i32,
    width: i32,
    max_color_value: u8,
    pixels: &Vec<(u8, u8, u8)>
) -> Result<(), Error> {
    let mut image_file = File::create(output_file_path)?;
    let resolution_header = width.to_string() + &" " + &height.to_string() + &"\n";
    let max_color_header = max_color_value.to_string() + &"\n";

    image_file.write_all(b"P3\n");
    image_file.write_all(resolution_header.as_bytes());
    image_file.write_all(max_color_header.as_bytes());
    for &(r, g, b) in pixels {
        let line = r.to_string() + &" " + &g.to_string() + &" " + &b.to_string() + &"\n";
        image_file.write_all(line.as_bytes());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use itertools::iproduct;

    use super::*;

    #[test]
    fn test_render_ppm_image_ascii() {
        let path = Path::new("file.ppm");
        let (w, h) = (256, 256);
        let pixels: Vec<(u8, u8, u8)> = iproduct!(0..=255, 0..=255)
            .map(|(j, i)| {
                let r = i as f64 / (w - 1) as f64;
                let g = j as f64 / (h - 1) as f64;
                let b = 0.0;

                ((255.999 * r) as u8, (255.999 * g) as u8, (255.999 * b) as u8)
            })
            .collect();

        render_ppm_image_ascii(path, h, w, 255, &pixels);
    }
}
