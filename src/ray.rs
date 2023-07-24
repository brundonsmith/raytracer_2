use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector<3>,
    pub direction: Vector<3>,
}
