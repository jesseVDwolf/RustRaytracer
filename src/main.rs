use std::{env, fmt};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;

use serde::Deserialize;

use raylib::prelude::*;
use itertools::iproduct;

/*
Goal of this Project is to build a RayTracer. A raytracer is a program
that shoots "rays" into a "scene" with "objects". If a ray hits an object
it can bounce of the object to form another ray. Rays are shot from a 
central location in a certain direction. The object positioned at this
location, aimed in a certain direction is called a "camera".

Ray tracing is a technique, just like ray casting, for rendering an image.
The aim here is to build the rendering engine using the ray tracing technique.

Glossary:
- ray
- scene
- camera
- object

Components:
- Window
    - WindowManager : https://docs.rs/penrose/latest/penrose/
- Geometry
    - Vector
    - Matrix
    - Quaternion
- Config
    - File handling
    - Parser
*/

mod sphere;
mod ppm;
mod vec;
mod point;
mod ray;
mod hit;
mod color;

use sphere::Sphere;
use vec::Vec3;
use point::Point3;
use ray::Ray;
use ppm::render_ppm_image_ascii;
use color::ColorRGBA;

#[derive(Debug, Deserialize, Clone)]
struct Config {
    version: String,
    objects: Vec<Sphere>
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let config = serde_json::from_reader(reader)?;

    // Return the `Config`
    Ok(config)
}

#[derive(Debug, Clone)]
struct ArgumentError;

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid arguments passed to program.")
    }
}

impl Error for ArgumentError {}


fn main() -> Result<(), Box<dyn Error>> {

    // 1sth argument should be a path object
    let args: Vec<String> = env::args().collect();
    dbg!(args.clone());

    if args.len() != 2 {
        return Err(Box::new(ArgumentError));
    }
    
    let p = &args[1];
    let config = read_config_from_file(p)?;

    dbg!(config.clone());

    // aspect ratio 16/9 (width to height)
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1024;

    let image_height = (image_width as f64 / aspect_ratio) as i32;
    // image height must be > 1

    // Camera information
    // The viewport is a 2D rectangle in front of the camera where
    // we are shooting our rays through. Its important that we define
    // its height and width using our choosen aspect ratio.
    let focal_length = 1.0;     // focal length is the length from origin to the viewport
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as i32 / image_height) as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // We need two vectors across horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // We also need two vectors that define that span the distance between two pixels.
    let pixel_delta_u = viewport_u / viewport_width;
    let pixel_delta_v = viewport_v / viewport_height;

    // Get a vector to the upper left pixel by using the focal lenght and our viewport vectors
    // then use our delta pixels to get the exact location of the pixel itself.
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u/2.0) - (viewport_v / 2.0);
    let starting_pixel = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let pixels: Vec<(u8, u8, u8)> = iproduct!(0..image_height, 0..image_width)
        .map(|(j, i)| {
            let pixel_center = starting_pixel + (i as f64 * pixel_delta_u) + (i as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(pixel_center, ray_direction);
            
            let color = ColorRGBA::white_blue_blend_over_y(&ray);
            (color.r as u8, color.g as u8, color.b as u8)
        })
        .collect();

    let output_file_path = Path::new("output.ppm");
    render_ppm_image_ascii(output_file_path, image_height, image_width, 255, &pixels);

    let ray = Ray::new(
        Vec3{x: 0.0, y: 0.0, z: 0.0},
        Vec3{x: 1.0, y: 0.0, z: 0.0}
    );
    let sphere = config.objects[0];
    let intersect = sphere.intersect(&ray);

    println!("intersect: {}", intersect);


    Ok(())
}
