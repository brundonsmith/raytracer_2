use std::f32::EPSILON;

use crate::{
    ray::Ray,
    shape::{Intersection, Shape},
    vector::Vector,
};

pub struct Plane {
    pub origin: Vector<3>,
    pub normal: Vector<3>,
}

impl Shape for Plane {
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
        let dot = self.normal.dot(ray.direction);

        if dot.abs() > EPSILON {
            let offset = ray.origin - self.origin;
            let distance = -1.0 * self.normal.dot(offset) / dot;

            Some(Intersection {
                position: ray.origin + ray.direction * distance,
                normal: self.normal,
            })
        } else {
            None
        }
    }
}

// ax + by + cz + d = 0
// 0x + 1y + 0z + 5 = 0

// 0 * (ax + t * bx) +
// 1 * (ay + t * by) +
// 0 * (az + t * bz)
// = -5

// r = a + tb
// x = ax + t * bx
// y = ay + t * by
// z = az + t * bz

// x = r * cos(u) * sin(v)
// y = r * sin(u) * sin(v)
// z = r * cos(v)

// t = (r * cos(u) * sin(v) - ox) / dx
// t = (r * sin(u) * sin(v) - oy) / dy
// t = (r * cos(v) - oz) / dz
