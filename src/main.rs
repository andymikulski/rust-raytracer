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
extern crate time;

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

    pub fn dot(one: &Vec3, two: &Vec3) -> f32 {
        return (one.x * two.x) + (one.y * two.y) + (one.z * two.z);
    }

    pub fn unit_vector(one: &Vec3) -> Vec3 {
        return one.get_unit_vector();
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

struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        return Sphere { center, radius };
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;

        let a = Vec3::dot(&r.direction, &r.direction);
        let b = 2.0 * Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp > t_min && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }

            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp > t_min && temp < t_max {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }

        return false;
    }
}

struct HitableList<'a> {
    list: Vec<&'a Hitable>,
}

impl<'a> HitableList<'a> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
        };

        let mut hit_anything = false;
        let mut closest_yet = t_max;

        for hitter in &self.list {
            let has_hit = hitter.hit(r, t_min, t_max, &mut temp_rec);
            if has_hit {
                hit_anything = true;
                if closest_yet > temp_rec.t {
                    closest_yet = temp_rec.t;
                    rec.normal = temp_rec.normal;
                    rec.p = temp_rec.p;
                    rec.t = temp_rec.t;
                }
            }
        }

        return hit_anything;
    }
}

//
//
// --- Actual program
//
//

fn get_ray_color(r: &Ray, hitables: &HitableList) -> Vec3 {
    let mut rec = HitRecord {
        t: 0.0,
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
    };

    if hitables.hit(r, 0.0, std::f32::MAX, &mut rec) {
        return (rec.normal + 1.0) * 0.5;
    } else {
        let unit_dir = r.direction.get_unit_vector();
        let t = (unit_dir.y + 1.0) * 0.5;
        return (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t);
    }

    // let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r);

    // if t > 0.0 {
    //     let temp = r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0);
    //     let n = Vec3::unit_vector(&temp);
    //     return (n + 1.0) * 0.5;
    // }

    // let unit_dir = r.direction.get_unit_vector();
    // let t = (unit_dir.y + 1.0) * 0.5;
    // return (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t);
}

fn main() {
    let start = time::get_time();
    let width: u32 = 600;
    let height: u32 = 300;
    let mut buffer = image::ImageBuffer::new(width, height);

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut collision_list = HitableList { list: vec![] };

    let world_objs = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    collision_list.list.push(&world_objs[0]);
    collision_list.list.push(&world_objs[1]);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let u: f32 = x as f32 / width as f32;
        let v: f32 = 1.0 - (y as f32 / height as f32);

        let dir = lower_left + horizontal * u + vertical * v;
        let r = Ray::new(&origin, &dir);

        let mut color = get_ray_color(&r, &collision_list);
        color *= 255.0;

        *pixel = image::Rgb([color.r(), color.g(), color.b()]);
    }

    let end = time::get_time();
    let start_ms = (start.sec as i64 * 1000) + (start.nsec as i64 / 1000 / 1000);
    let end_ms = (end.sec as i64 * 1000) + (end.nsec as i64 / 1000 / 1000);

    println!("time taken: {}ms", end_ms - start_ms);

    // Save the image as “fractal.png”, the format is deduced from the path
    buffer.save("fractal.png").unwrap();
}
