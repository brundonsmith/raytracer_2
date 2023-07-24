use crate::{plane::Plane, ray::Ray, sphere::Sphere, vector::Vector};

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub position: Vector<3>,
    pub normal: Vector<3>,
}

pub enum ShapeEnum {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape for ShapeEnum {
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
        match self {
            ShapeEnum::Sphere(s) => s.intersection(ray),
            ShapeEnum::Plane(s) => s.intersection(ray),
        }
    }
}

pub trait Shape {
    fn intersection(&self, ray: Ray) -> Option<Intersection>;
}

impl Into<ShapeEnum> for Sphere {
    fn into(self) -> ShapeEnum {
        ShapeEnum::Sphere(self)
    }
}

impl Into<ShapeEnum> for Plane {
    fn into(self) -> ShapeEnum {
        ShapeEnum::Plane(self)
    }
}
