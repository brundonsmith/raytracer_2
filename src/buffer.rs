use std::ops::{Index, IndexMut};

use crate::constants::{RESOLUTION_X, RESOLUTION_Y};

#[derive(Debug, Clone, Copy)]
pub struct Buffer<T: Clone + Copy> {
    data: [[T; RESOLUTION_Y]; RESOLUTION_X],
}

impl<T: Clone + Copy> Buffer<T> {
    pub fn new(initial: T) -> Self {
        Self {
            data: [[initial; RESOLUTION_Y]; RESOLUTION_X],
        }
    }
}

impl<T: Clone + Copy> Index<usize> for Buffer<T> {
    type Output = [T; RESOLUTION_Y];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Clone + Copy> IndexMut<usize> for Buffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
