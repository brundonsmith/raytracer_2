#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color(pub f32, pub f32, pub f32);

impl Default for Color {
    fn default() -> Self {
        Self(Default::default(), Default::default(), Default::default())
    }
}
