#![allow(unstable)]

use std::ops::{Index, IndexMut};
use std::num::Float;

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix4([[f32; 4]; 4]);

impl Index<usize> for Matrix4 {
    type Output = [f32; 4];

    fn index<'a>(&'a self, row: &usize) -> &'a [f32; 4] {
        &self.0[*row]
    }
}

impl IndexMut<usize> for Matrix4 {
    type Output = [f32; 4];

    fn index_mut<'a>(&'a mut self, row: &usize) -> &'a mut [f32; 4] {
        &mut self.0[*row]
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

    pub fn scale_by_vector(&mut self, v: &Vector3) {
        self.scale(v[0], v[1], v[2])
    }


	/// Builds a translation 4 * 4 matrix created from a vector of 3 components.
	/// Input matrix multiplied by this translation matrix.
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.0[3][0] += self.0[0][0] * x + self.0[1][0] * y + self.0[2][0] * z;
        self.0[3][1] += self.0[0][1] * x + self.0[1][1] * y + self.0[2][1] * z;
        self.0[3][2] += self.0[0][2] * x + self.0[1][2] * y + self.0[2][2] * z;
    }

    pub fn translate_by_vector(&mut self, v: &Vector3) {
        self.translate(v[0], v[1], v[2])
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Vector3([f32; 3]);

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index<'a>(&'a self, index: &usize) -> &'a f32 {
        &self.0[*index]
    }
}

impl IndexMut<usize> for Vector3 {
    type Output = f32;

    fn index_mut<'a>(&'a mut self, index: &usize) -> &'a mut f32 {
        &mut self.0[*index]
    }
}

impl Vector3 {
    pub fn as_array(&self) -> &[f32; 3] {
        &self.0
    }

    pub fn from_array(array: [f32; 3]) -> Self {
        Vector3(array)
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3::from_array([x, y, z])
    }

    pub fn from_scalar(scalar: f32) -> Self {
        Vector3::from_array([scalar, scalar, scalar])
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn distance(&self, other: &Vector3) -> f32 {
        let mut result = other.clone();
        result.subtract(self);
        result.length()
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.0[0] * other.0[0] +
        self.0[1] * other.0[1] +
        self.0[2] * other.0[2]
    }

    /// returns the cross product
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
			self.0[1] * other.0[2] - other.0[1] * self.0[2],
			self.0[2] * other.0[0] - other.0[2] * self.0[0],
			self.0[0] * other.0[1] - other.0[0] * self.0[1])
    }

    pub fn normalize(&mut self) {
        let value = self.dot(self).rsqrt();
        self.scale(value);
    }

    pub fn subtract(&mut self, other: &Vector3) {
        let mut result = other.clone();
        result.negate();
        self.add(&result);
    }

    pub fn add(&mut self, other: &Vector3) {
        self.0[0] += other.0[0];
        self.0[1] += other.0[1];
        self.0[2] += other.0[2];
    }

    pub fn negate(&mut self) {
        self.0[0] = -self.0[0];
        self.0[1] = -self.0[1];
        self.0[2] = -self.0[2];
    }

    pub fn scale(&mut self, scalar: f32) {
        self.0[0] *= scalar;
        self.0[1] *= scalar;
        self.0[2] *= scalar;
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
    m.scale_by_vector(&Vector3::new(2.0, 4.0, 3.0));
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
