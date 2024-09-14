use std::{env, fmt};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;

use serde::Deserialize;

use raylib::prelude::*;
use itertools::{iproduct, Itertools};

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
mod plane;
mod ppm;
mod vec;
mod point;
mod ray;
mod hit;
mod color;
mod traits;

use sphere::Sphere;
use plane::Plane;
use vec::Vec3;
use point::Point3;
use ray::Ray;
use color::RGBAColor;
use traits::Intersectable;

#[derive(Debug, Deserialize, Clone)]
struct Config {
    version: String,
    spheres: Vec<Sphere>,
    planes: Vec<Plane>
}

fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let config: Config = serde_json::from_reader(reader)?;

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


struct Window {
    // The window represents the screen that shows all pixels.
    // It is created by providing a width. The aspect ratio is 
    // used to calculate a proper height
    pub width: i32,
    pub height: i32
}

impl Window {
    pub fn new(width: i32) -> Self {
        // aspect ratio 16/9 (width to height)
        let aspect_ratio: f64 = 16.0 / 9.0;

        let height = (width as f64 / aspect_ratio) as i32;
        assert!(height > 1);
        Self { width, height }
    }
}


struct Camera {
    // The camera is the location from which the rays are shot.
    // Each ray shot through the viewport originates at the camera
    // origin.
    location: Point3
}

impl Camera {
    pub fn new(location: Point3) -> Self {
        Self { location }
    }
}


struct Viewport {
    // The viewport described the small window through which
    // the rays are shot into the world. This is a 2D plane in
    // front of the camera.
    starting_pixel: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3
}

impl Viewport {
    pub fn new(window: &Window, camera: &Camera) -> Self {
        // The viewport is a 2D rectangle in front of the camera where
        // we are shooting our rays through. Its important that we define
        // its height and width using our choosen aspect ratio.
        let focal_length = 1.0;     // focal length is the length from origin to the viewport
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (window.width as f64 / window.height as f64);

        // We need two vectors across horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // We also need two vectors that define that span the distance between two pixels.
        let pixel_delta_u = viewport_u / window.width as f64;
        let pixel_delta_v = viewport_v / window.height as f64;

        // Get a vector to the upper left pixel by using the focal lenght and our viewport vectors
        // then use our delta pixels to get the exact location of the pixel itself.
        let viewport_upper_left = camera.location - Vec3::new(0.0, 0.0, focal_length) - (viewport_u/2.0) - (viewport_v / 2.0);
        let starting_pixel = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Self { starting_pixel, pixel_delta_u, pixel_delta_v }
    }
}

fn move_camera_on_key_press(rl: &RaylibHandle, camera: &mut Camera) {
    let step_size = 0.5;

    if rl.is_key_pressed(KeyboardKey::KEY_W) || rl.is_key_pressed(KeyboardKey::KEY_UP) {
        println!("Moving forward.");
        camera.location.z += step_size;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_S) || rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
        println!("Moving backwards.");
        camera.location.z -= step_size;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_A) || rl.is_key_pressed(KeyboardKey::KEY_LEFT){
        println!("Moving to the left.");
        camera.location.x += step_size;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_D) || rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
        println!("Moving to the right.");
        camera.location.x -= step_size;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        println!("Moving up.");
        camera.location.y -= step_size;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_LEFT_SHIFT) {
        println!("Moving down.");
        camera.location.y += step_size;
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    // 1sth argument should be a path object
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Box::new(ArgumentError));
    }
    
    let p = &args[1];
    let config = read_config_from_file(p)?;

    let image_width = 1024;
    let window = Window::new(image_width);
    let mut camera = Camera::new(Point3::new(0.0, 0.0, 0.0));
    let viewport = Viewport::new(&window, &camera);
 
    let (mut rl, thread) = raylib::init()
        .size(window.width, window.height)
        .title("Raytracer")
        .build();

    let mut intersectables: Vec<Box<dyn Intersectable>> = Vec::new();
    for sphere in config.spheres {
        intersectables.push(Box::new(sphere));
    }
    for plane in config.planes {
        intersectables.push(Box::new(plane));
    }
     
    while !rl.window_should_close() {

        // first check if any key was pressed. If so, update the camera position.
        move_camera_on_key_press(&rl, &mut camera);

        let mut d = rl.begin_drawing(&thread);

        // clear the display
        d.clear_background(Color::WHITE);


        // shoot each ray into the scene and check what the returned color looks like
        for (y, x) in iproduct!(0..window.height, 0..window.width) {
            let pixel_center = viewport.starting_pixel + (x as f64 * viewport.pixel_delta_u) + (y as f64 * viewport.pixel_delta_v);
            let ray_direction = pixel_center - camera.location;
            let ray = Ray::new(camera.location, ray_direction);

            // TODO find() short-circuits meaning you could have objects
            // behind one another and render the wrong one.
            let intersectable = intersectables
                .iter()
                .find(|o| o.intersect(&ray));
            
            let color = match intersectable {
                Some(object) => {
                    let hits = object.intersect_hits(&ray).unwrap();
                    let h = hits.first().unwrap();

                    // for now a nice color created using the normal
                    if h.t > 0.0 {
                        let c = RGBAColor::new(
                            h.normal.x as f32 + 1.0,
                            h.normal.y as f32 + 1.0,
                            h.normal.z as f32 + 1.0
                        ).unwrap();
                        c * 0.5 * 255.99
                    }
                    else {
                        RGBAColor::white_blue_blend_over_y(&ray)
                    }
                },
                None => {
                    RGBAColor::white_blue_blend_over_y(&ray)
                }
            };
            d.draw_pixel(x, y, color);
        }
    }

    Ok(())
}
