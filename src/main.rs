/*
rust questions:
 - importing local files?? `mod` examples seem way too complicated
 - overloading methods, specifically `new`? `new` vs `new_init`..?
 - operator overloading seems like a lot of dupe code?
  - i.e. `impl ops::Mul<f32> for Vec3` means (f32 * Vec3) but NOT (Vec3 * f32) ??
  - YEP. noncommutative multiplication exists, such as for matrices
  - see https://github.com/scottmcm/pinteger/blob/c2813047aa978a2787131d3369bc4eec6b8e8cb9/src/lib.rs#L27-L46 for a macro example to get around dupe code
*/
#![allow(dead_code)]
extern crate image;

use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 { x, y, z };
    }

    pub fn r(&self) -> u8 {
        return self.x as u8;
    }
    pub fn g(&self) -> u8 {
        return self.y as u8;
    }
    pub fn b(&self) -> u8 {
        return self.z as u8;
    }

    pub fn length(&self) -> f32 {
        return (self.squared_length()).sqrt();
    }

    pub fn squared_length(&self) -> f32 {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)) as f32;
    }

    pub fn get_unit_vector(&self) -> Vec3 {
        let k = 1.0 / self.length();
        return Vec3::new(self.x * k, self.y * k, self.z * k);
    }

    pub fn to_unit_vector(mut self) -> Vec3 {
        self = self.get_unit_vector();
        return self;
    }

    pub fn dot(one: &Vec3, two: &Vec3) -> f32 {
        return (one.x * two.x) + (one.y * two.y) + (one.z * two.z);
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        return Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            -(self.x * rhs.z - self.z * rhs.x),
            self.x * rhs.y - self.y * rhs.x,
        );
    }
}

// Add
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}
impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl ops::Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Vec3 {
        return Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs);
    }
}
impl ops::AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

// Subtract
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}
impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Vec3 {
        return Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs);
    }
}
impl ops::SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

// Divide
impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z);
    }
}
impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        return Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// Multiply
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z);
    }
}
impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        return Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        return Ray {
            origin: *origin,
            direction: *direction,
        };
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        return self.origin + (self.direction * t);
    }
}

//
//
// --- Actual program
//
//

fn get_ray_color(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let unit_dir = r.direction.get_unit_vector();
    let t: f32 = 0.5 * (unit_dir.y + 1.0);

    return (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t);
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin - *center;

    let a = Vec3::dot(&r.direction, &r.direction);
    let b = 2.0 * Vec3::dot(&oc, &r.direction);
    let c = Vec3::dot(&oc, &oc) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);

    return discriminant > 0.0;
}

fn main() {
    let width: u32 = 200;
    let height: u32 = 100;
    let mut buffer = image::ImageBuffer::new(width, height);

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let u: f32 = x as f32 / width as f32;
        let v: f32 = y as f32 / height as f32;

        let dir = lower_left + horizontal * u + vertical * v;
        let r = Ray::new(&origin, &dir);

        let mut color = get_ray_color(&r);
        color *= 255.0;

        *pixel = image::Rgb([color.r(), color.g(), color.b()]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    buffer.save("fractal.png").unwrap();
}
