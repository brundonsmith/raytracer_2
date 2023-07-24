#![allow(dead_code)]

use std::{sync::Mutex, time::SystemTime};

use rayon::{iter::IntoParallelIterator, prelude::ParallelIterator};

use buffer::Buffer;
use color::Color;
use constants::{RESOLUTION_X, RESOLUTION_X_FLOAT, RESOLUTION_Y_FLOAT, TOTAL_PIXELS};
use ray::Ray;
use shape::{Intersection, Shape};
use sphere::Sphere;
use vector::Vector;

use crate::{illumination::Illumination, material::Material, plane::Plane, shape::ShapeEnum};

mod buffer;
mod color;
mod constants;
mod illumination;
mod material;
mod matrix;
mod plane;
mod ray;
mod shape;
mod sphere;
mod utils;
mod vector;
mod write_image;

const CHUNKS: usize = 100;
const CHUNK_SIZE: usize = TOTAL_PIXELS / CHUNKS;

fn main() {
    let shapes_and_materials: Vec<(ShapeEnum, Material)> = vec![
        (
            Sphere {
                position: Vector::from([0.0, 0.0, 10.0]),
                radius: 3.0,
            }
            .into(),
            Material::Diffuse {
                color: Color(0.0, 1.0, 0.0),
            },
        ),
        (
            Plane {
                origin: Vector::from([0.0, -15.0, 0.0]),
                normal: Vector::from([0.0, 1.0, -1.0]).normalized(),
            }
            .into(),
            Material::Emissive {
                emission: Illumination {
                    color: Color(1.0, 0.0, 0.0),
                    lum: 1.0,
                },
            },
        ),
        (
            Plane {
                origin: Vector::from([0.0, 0.0, 15.0]),
                normal: Vector::from([0.0, 0.0, -1.0]),
            }
            .into(),
            Material::Emissive {
                emission: Illumination {
                    color: Color(0.0, 0.0, 1.0),
                    lum: 1.0,
                },
            },
        ),
    ];

    let light_direction: Vector<3> = [1.0, 1.0, -1.0].into();

    let start = SystemTime::now();

    let buf = Mutex::new(Buffer::new(Color::default()));
    (0..CHUNKS).into_par_iter().for_each(|chunk| {
        let chunk_start = chunk * CHUNK_SIZE;

        let mut chunk_buf = [Color::default(); CHUNK_SIZE];

        for i in 0..CHUNK_SIZE {
            let x = (i + chunk_start) % RESOLUTION_X;
            let y = (i + chunk_start) / RESOLUTION_X;

            let ray = pixel_to_ray(x, y);

            let hit = shapes_and_materials
                .iter()
                .filter_map(|(shape, material)| shape.intersection(ray).map(|i| (i, material)))
                .fold(None, |acc, (intersection, material)| {
                    let distance = (intersection.position - ray.origin).magnitude();

                    match acc {
                        Some((prev_distance, i)) => {
                            if distance < prev_distance {
                                Some((distance, (intersection, material)))
                            } else {
                                Some((prev_distance, i))
                            }
                        }
                        None => Some((distance, (intersection, material))),
                    }
                })
                .map(|(_, i)| i);

            chunk_buf[i] = match hit {
                Some((
                    Intersection {
                        position: _,
                        normal,
                    },
                    material,
                )) => match material {
                    Material::Emissive { emission } => emission.color,
                    Material::Specular { smoothness } => todo!(),
                    Material::Diffuse { color } => *color,
                },
                None => Color(0.0, 0.0, 0.0),
            };
        }

        let mut lock = buf.lock().unwrap();
        for i in 0..CHUNK_SIZE {
            let x = (i + chunk_start) % RESOLUTION_X;
            let y = (i + chunk_start) / RESOLUTION_X;

            (*lock)[x][y] = chunk_buf[i];
        }
    });

    println!(
        "{}ms",
        SystemTime::now().duration_since(start).unwrap().as_millis()
    );

    write_image::write_image(buf.into_inner().unwrap(), "output.png");
}

fn pixel_to_ray(x: usize, y: usize) -> Ray {
    Ray {
        origin: [
            (x as f32 - RESOLUTION_X_FLOAT / 2.0) / 10.0,
            (y as f32 - RESOLUTION_Y_FLOAT / 2.0) / 10.0 * -1.0,
            0.0,
        ]
        .into(),
        direction: [0.0, 0.0, 1.0].into(),
    }
}
