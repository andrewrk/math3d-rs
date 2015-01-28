#![allow(unstable)]

use std::ops::Index;

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix4([[f32; 4]; 4]);

impl Index<usize> for Matrix4 {
    type Output = [f32; 4];

    fn index<'a>(&'a self, row: &usize) -> &'a [f32; 4] {
        &self.0[*row]
    }
}

impl Matrix4 {
    pub fn as_array(&self) -> &[[f32; 4]; 4] {
        &self.0
    }

    pub fn from_array(array: [[f32; 4]; 4]) -> Self {
        Matrix4(array)
    }

    pub fn identity() -> Self {
        Matrix4::from_array([[1.0, 0.0, 0.0, 0.0],
                          [0.0, 1.0, 0.0, 0.0],
                          [0.0, 0.0, 1.0, 0.0],
                          [0.0, 0.0, 0.0, 1.0]])
    }

    pub fn operate<F: Fn(f32) -> f32>(&mut self, f: F) {
        self.0[0][0] = f(self.0[0][0]);
        self.0[0][1] = f(self.0[0][1]);
        self.0[0][2] = f(self.0[0][2]);
        self.0[0][3] = f(self.0[0][3]);

        self.0[1][0] = f(self.0[1][0]);
        self.0[1][1] = f(self.0[1][1]);
        self.0[1][2] = f(self.0[1][2]);
        self.0[1][3] = f(self.0[1][3]);

        self.0[2][0] = f(self.0[2][0]);
        self.0[2][1] = f(self.0[2][1]);
        self.0[2][2] = f(self.0[2][2]);
        self.0[2][3] = f(self.0[2][3]);

        self.0[3][0] = f(self.0[3][0]);
        self.0[3][1] = f(self.0[3][1]);
        self.0[3][2] = f(self.0[3][2]);
        self.0[3][3] = f(self.0[3][3]);
    }

    pub fn scale(&mut self, scalar: f32) {
        self.operate( |x| { x * scalar } )
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[test]
fn test_matrix_to_array() {
    let array = {
        let foo = Matrix4::from_array([[1.0, 0.0, 0.0, 0.0],
                                       [0.0, 1.0, 0.0, 0.0],
                                       [0.0, 0.0, 1.0, 0.0],
                                       [0.0, 0.0, 0.0, 1.0]]);
        assert_eq!(foo.as_array()[0][0], 1.0);
        *foo.as_array()
    };
    assert_eq!(array[1][1], 1.0);
}

#[test]
fn test_scale() {
    let mut m = Matrix4::identity();
    assert_eq!(m[2][2], 1.0);
    m.scale(3.0);
    assert_eq!(m[2][2], 3.0);
}
