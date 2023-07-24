use crate::{color::Color, illumination::Illumination};

pub enum Material {
    Emissive { emission: Illumination },
    Specular { smoothness: f32 },
    Diffuse { color: Color },
}
