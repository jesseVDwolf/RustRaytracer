use std::ops;
use raylib::ffi::Color;

use crate::ray::Ray;


#[derive(Debug, Clone, Copy)]
pub struct RGBAColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Into<Color> for RGBAColor {
    fn into(self) -> Color {
        Color {
            r: self.r as u8,
            g: self.g as u8,
            b: self.b as u8,
            a: self.a as u8
        }
    }
}

impl RGBAColor {
    
    pub fn new(r: f32, g: f32, b: f32) -> Option<Self> {
        if vec![r, g, b].into_iter().any(|x| x < 0.0 || x > 255.0) {
            None
        } else {
            Some(Self{r, g, b, a: 255.0})
        }
    }

    pub fn white_blue_blend_over_y(r: &Ray) -> Self {
        let unit = r.direction.normalise();
        let a = 0.5 * (unit.y + 1.0) as f32;
        let white = Self::new(1.0, 1.0, 1.0).unwrap();
        let blue = Self::new(0.3, 0.5, 1.0).unwrap();
        ((1.0 - a) * white + a * blue).unwrap() * 255.999
    }

    pub fn as_rgb_tuple(self) -> (u8, u8, u8) {
        (self.r as u8, self.g as u8, self.b as u8)
    }
}

impl ops::Mul<f32> for RGBAColor {
    type Output = RGBAColor;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a
        }
    }
}

impl ops::Mul<RGBAColor> for f32 {
    type Output = RGBAColor;

    fn mul(self, rhs: RGBAColor) -> Self::Output {
        rhs * self
    }
}

impl ops::Add<RGBAColor> for RGBAColor {
    type Output = Option<RGBAColor>;

    fn add(self, rhs: RGBAColor) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    use itertools::iproduct;

    use super::*;

    use crate::sphere::Sphere;
    use crate::vec::Vec3;
    use crate::point::Point3;
    use crate::ppm::render_ppm_image_ascii;

    #[test]
    fn test_color() {
        // aspect ratio 16/9 (width to height)
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;

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
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Get a vector to the upper left pixel by using the focal lenght and our viewport vectors
        // then use our delta pixels to get the exact location of the pixel itself.
        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u/2.0) - (viewport_v / 2.0);
        let starting_pixel = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixels: Vec<(u8, u8, u8)> = iproduct!(0..image_height, 0..image_width)
            .map(|(j, i)| {
                let pixel_center = starting_pixel + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
                let ray_direction = pixel_center - camera_center;
                let ray = Ray::new(camera_center, ray_direction);
                
                let color = RGBAColor::white_blue_blend_over_y(&ray);
                (color.r as u8, color.g as u8, color.b as u8)
            })
            .collect();
    
        let output_file_path = Path::new("white_to_blue_gradient.ppm");
        render_ppm_image_ascii(output_file_path, image_height, image_width, 255, &pixels);
    }

    #[test]
    fn test_color_with_sphere() {
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
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Get a vector to the upper left pixel by using the focal lenght and our viewport vectors
        // then use our delta pixels to get the exact location of the pixel itself.
        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u/2.0) - (viewport_v / 2.0);
        let starting_pixel = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let rays: Vec<Ray> = iproduct!(0..image_height, 0..image_width)
            .map(|(i, j)|{
                let pixel_center = starting_pixel + (j as f64 * pixel_delta_u) + (i as f64 * pixel_delta_v);
                let ray_direction = pixel_center - camera_center;
                Ray::new(camera_center, ray_direction)
            })
            .collect();

        let objects = vec![
            Sphere{ origin: Point3{ x: 0.0, y: 0.0, z: -5.0 }, radius: 1.0}
        ];

        let mut pixels: Vec<(u8, u8, u8)> = vec![];
        for r in &rays {
            for o in &objects {
                let color = match o.intersect_hits(r) {
                    None => RGBAColor::white_blue_blend_over_y(r),
                    Some(hits) => {
                        let first_hit = hits.first().unwrap();
                        if first_hit.t > 0.0 {
                            RGBAColor::new
                            (
                                first_hit.normal.x as f32 + 1.0,
                                first_hit.normal.y as f32+ 1.0,
                                first_hit.normal.z as f32 + 1.0
                            ).unwrap() *
                            0.5 *
                            255.99
                        }
                        else {
                            RGBAColor::white_blue_blend_over_y(r)
                        }
                    }
                };
                pixels.push(color.as_rgb_tuple())
            }
        }

        let output_file_path = Path::new("circle.ppm");
        render_ppm_image_ascii(output_file_path, image_height, image_width, 255, &pixels);
    }
}
