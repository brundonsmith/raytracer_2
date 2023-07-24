use crate::{
    ray::Ray,
    shape::{Intersection, Shape},
    vector::Vector,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Sphere {
    pub position: Vector<3>,
    pub radius: f32,
}

impl Shape for Sphere {
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
        let origin_to_center = self.position - ray.origin;
        let origin_to_center_distance = origin_to_center.magnitude();
        let origin_to_center_normalized = origin_to_center.normalized();
        let projection = ray.direction.dot(origin_to_center_normalized);
        let closest_point = ray.origin + ray.direction * (projection * origin_to_center_distance);
        let closest_point_to_center = closest_point - self.position;
        let distance_to_center = closest_point_to_center.magnitude();

        if distance_to_center <= self.radius {
            let to_surface =
                (self.radius * self.radius - distance_to_center * distance_to_center).sqrt();
            let intersect_position =
                ray.origin + ray.direction * (closest_point.magnitude() - to_surface);

            Some(Intersection {
                position: intersect_position,
                normal: (intersect_position - self.position).normalized(),
            })
        } else {
            None
        }
    }
}

// const intersectSphere = (line, sphere) => {
// 	   const originToSphere = sub(sphere.position, line.origin)
//     const originToSphereDistance = magnitude(originToSphere)
//     const originToSphereNormalized = normalized(originToSphere)
//     const projected = dot(originToSphereNormalized, line.direction)
//     const distanceToNearestPoint = projected * originToSphereDistance
//     const nearestPoint = add(line.origin, scale(line.direction, distanceToNearestPoint))

//     const nearestPointToCenter = magnitude(sub(nearestPoint, sphere.position))

//     if (nearestPointToCenter <= sphere.radius) {
//         const nearestPointToSurface = Math.sqrt(sphere.radius * sphere.radius - nearestPointToCenter * nearestPointToCenter)
//         return add(line.origin, scale(line.direction, distanceToNearestPoint - nearestPointToSurface))
//     }
// }
