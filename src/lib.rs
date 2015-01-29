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


	/// Builds a scale 4 * 4 matrix created from 3 scalars.
	/// Input matrix multiplied by this scale matrix.
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.0[0][0] *= x;
        self.0[0][1] *= x;
        self.0[0][2] *= x;
        self.0[0][3] *= x;

        self.0[1][0] *= y;
        self.0[1][1] *= y;
        self.0[1][2] *= y;
        self.0[1][3] *= y;

        self.0[2][0] *= z;
        self.0[2][1] *= z;
        self.0[2][2] *= z;
        self.0[2][3] *= z;
    }

    pub fn scale_by_vector(&mut self, v: &Vector4) {
        self.scale(v[0], v[1], v[2])
    }


	/// Builds a translation 4 * 4 matrix created from a vector of 3 components.
	/// Input matrix multiplied by this translation matrix.
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.0[3][0] += self.0[0][0] * x + self.0[1][0] * y + self.0[2][0] * z;
        self.0[3][1] += self.0[0][1] * x + self.0[1][1] * y + self.0[2][1] * z;
        self.0[3][2] += self.0[0][2] * x + self.0[1][2] * y + self.0[2][2] * z;
    }

    pub fn translate_by_vector(&mut self, v: &Vector4) {
        self.translate(v[0], v[1], v[2])
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Vector4([f32; 4]);

impl Index<usize> for Vector4 {
    type Output = f32;

    fn index<'a>(&'a self, index: &usize) -> &'a f32 {
        &self.0[*index]
    }
}

impl Vector4 {
    pub fn as_array(&self) -> &[f32; 4] {
        &self.0
    }

    pub fn from_array(array: [f32; 4]) -> Self {
        Vector4(array)
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector4::from_array([x, y, z, 1.0])
    }

    pub fn from_scalar(scalar: f32) -> Self {
        Vector4::from_array([scalar, scalar, scalar, 1.0])
    }
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
    m.scale(3.0, 3.0, 3.0);
    assert_eq!(m[2][2], 3.0);
    m.scale_by_vector(&Vector4::new(2.0, 4.0, 3.0));
    assert_eq!(m[2][2], 9.0);
}

#[test]
fn test_translate() {
    let mut m = Matrix4::identity();
    m.translate(1.0, 2.0, 3.0);
    assert_eq!(m[3][0], 1.0);
    assert_eq!(m[3][1], 2.0);
    assert_eq!(m[3][2], 3.0);
}
