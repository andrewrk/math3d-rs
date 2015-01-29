#![feature(std_misc)]
#![feature(core)]

use std::ops::{Index, IndexMut};
use std::num::Float;
use std::fmt;
use std::result::Result;

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

impl fmt::Debug for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}, {:?}, {:?}]\n[{:?}, {:?}, {:?}, {:?}]\n[{:?}, {:?}, {:?}, {:?}]\n[{:?}, {:?}, {:?}, {:?}]\n",
                self.0[0][0], self.0[0][1], self.0[0][2], self.0[0][3],
                self.0[1][0], self.0[1][1], self.0[1][2], self.0[1][3],
                self.0[2][0], self.0[2][1], self.0[2][2], self.0[2][3],
                self.0[3][0], self.0[3][1], self.0[3][2], self.0[3][3])
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

    /// Creates a matrix for an orthographic parallel viewing volume.
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let mut m = Matrix4::identity();
        m.0[0][0] = 2.0 / (right - left);
        m.0[1][1] = 2.0 / (top - bottom);
        m.0[2][2] = -1.0;
        m.0[3][0] = -(right + left) / (right - left);
        m.0[3][1] = -(top + bottom) / (top - bottom);
        m
    }

    /// Builds a scale 4 * 4 matrix created from 3 scalars.
    /// Input matrix multiplied by this scale matrix.
    pub fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        Matrix4::from_array([[self.0[0][0] * x, self.0[0][1] * y, self.0[0][2] * z, self.0[0][3]],
                             [self.0[1][0] * x, self.0[1][1] * y, self.0[1][2] * z, self.0[1][3]],
                             [self.0[2][0] * x, self.0[2][1] * y, self.0[2][2] * z, self.0[2][3]],
                             [self.0[3][0] * x, self.0[3][1] * y, self.0[3][2] * z, self.0[3][3]]])
    }

    pub fn scale_by_vector(&self, v: &Vector3) -> Self {
        self.scale(v[0], v[1], v[2])
    }

    /// Builds a translation 4 * 4 matrix created from a vector of 3 components.
    /// Input matrix multiplied by this translation matrix.
    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        Matrix4::from_array([[self.0[0][0], self.0[0][1], self.0[0][2], self.0[0][3] + self.0[0][0] * x + self.0[0][1] * y + self.0[0][2] * z],
                             [self.0[1][0], self.0[1][1], self.0[1][2], self.0[1][3] + self.0[1][0] * x + self.0[1][1] * y + self.0[1][2] * z],
                             [self.0[2][0], self.0[2][1], self.0[2][2], self.0[2][3] + self.0[2][0] * x + self.0[2][1] * y + self.0[2][2] * z],
                             [self.0[3][0], self.0[3][1], self.0[3][2], self.0[3][3]]])

    }

    pub fn translate_by_vector(&self, v: &Vector3) -> Self {
        self.translate(v[0], v[1], v[2])
    }

    pub fn transpose(&self) -> Self {
        Matrix4::from_array([[self.0[0][0], self.0[1][0], self.0[2][0], self.0[3][0]],
                             [self.0[0][1], self.0[1][1], self.0[2][1], self.0[3][1]],
                             [self.0[0][2], self.0[1][2], self.0[2][2], self.0[3][2]],
                             [self.0[0][3], self.0[1][3], self.0[2][3], self.0[3][3]]])
    }

    /// matrix multiplication
    pub fn mult(&self, other: &Matrix4) -> Matrix4 {
        Matrix4::from_array([
           [
             self.0[0][0]*other.0[0][0] + self.0[0][1]*other.0[1][0] + self.0[0][2]*other.0[2][0] + self.0[0][3]*other.0[3][0],
             self.0[0][0]*other.0[0][1] + self.0[0][1]*other.0[1][1] + self.0[0][2]*other.0[2][1] + self.0[0][3]*other.0[3][1],
             self.0[0][0]*other.0[0][2] + self.0[0][1]*other.0[1][2] + self.0[0][2]*other.0[2][2] + self.0[0][3]*other.0[3][2],
             self.0[0][0]*other.0[0][3] + self.0[0][1]*other.0[1][3] + self.0[0][2]*other.0[2][3] + self.0[0][3]*other.0[3][3],
           ],
           [
             self.0[1][0]*other.0[0][0] + self.0[1][1]*other.0[1][0] + self.0[1][2]*other.0[2][0] + self.0[1][3]*other.0[3][0],
             self.0[1][0]*other.0[0][1] + self.0[1][1]*other.0[1][1] + self.0[1][2]*other.0[2][1] + self.0[1][3]*other.0[3][1],
             self.0[1][0]*other.0[0][2] + self.0[1][1]*other.0[1][2] + self.0[1][2]*other.0[2][2] + self.0[1][3]*other.0[3][2],
             self.0[1][0]*other.0[0][3] + self.0[1][1]*other.0[1][3] + self.0[1][2]*other.0[2][3] + self.0[1][3]*other.0[3][3],
           ],
           [
             self.0[2][0]*other.0[0][0] + self.0[2][1]*other.0[1][0] + self.0[2][2]*other.0[2][0] + self.0[2][3]*other.0[3][0],
             self.0[2][0]*other.0[0][1] + self.0[2][1]*other.0[1][1] + self.0[2][2]*other.0[2][1] + self.0[2][3]*other.0[3][1],
             self.0[2][0]*other.0[0][2] + self.0[2][1]*other.0[1][2] + self.0[2][2]*other.0[2][2] + self.0[2][3]*other.0[3][2],
             self.0[2][0]*other.0[0][3] + self.0[2][1]*other.0[1][3] + self.0[2][2]*other.0[2][3] + self.0[2][3]*other.0[3][3],
           ],
           [
             self.0[3][0]*other.0[0][0] + self.0[3][1]*other.0[1][0] + self.0[3][2]*other.0[2][0] + self.0[3][3]*other.0[3][0],
             self.0[3][0]*other.0[0][1] + self.0[3][1]*other.0[1][1] + self.0[3][2]*other.0[2][1] + self.0[3][3]*other.0[3][1],
             self.0[3][0]*other.0[0][2] + self.0[3][1]*other.0[1][2] + self.0[3][2]*other.0[2][2] + self.0[3][3]*other.0[3][2],
             self.0[3][0]*other.0[0][3] + self.0[3][1]*other.0[1][3] + self.0[3][2]*other.0[2][3] + self.0[3][3]*other.0[3][3],
           ],
        ])
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

impl fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?}, {:?}]", self.0[0], self.0[1], self.0[2])
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
        Vector3::new(scalar, scalar, scalar)
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn distance(&self, other: &Vector3) -> f32 {
        other.subtract(self).length()
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

    pub fn normalize(&self) -> Self {
        self.scale(self.dot(self).rsqrt())
    }

    pub fn subtract(&self, other: &Vector3) -> Self {
        self.add(&other.negate())
    }

    pub fn add(&self, other: &Vector3) -> Self {
        Vector3::new(self.0[0] + other.0[0],
                     self.0[1] + other.0[1],
                     self.0[2] + other.0[2])
    }

    pub fn negate(&self) -> Self {
        Vector3::new(-self.0[0],
                     -self.0[1],
                     -self.0[2])
    }

    pub fn scale(&self, scalar: f32) -> Self {
        Vector3::new(self.0[0] * scalar,
                     self.0[1] * scalar,
                     self.0[2] * scalar)
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
    let m = Matrix4::from_array([
            [0.840188, 0.911647, 0.277775, 0.364784],
            [0.394383, 0.197551, 0.55397, 0.513401],
            [0.783099, 0.335223, 0.477397, 0.95223],
            [0.79844, 0.76823, 0.628871, 0.916195]]);
    let expected = Matrix4::from_array([
            [0.118973, 0.653922, 0.176585, 0.364784],
            [0.0558456, 0.141703, 0.352165, 0.513401],
            [0.110889, 0.240454, 0.303487, 0.95223],
            [0.113061, 0.551049, 0.399781, 0.916195]]);
    let answer = m.scale(0.141603, 0.717297, 0.635712);
    println!("answer:\n{:?}", answer);
    assert_matrix_eq(&answer, &expected);
}

#[test]
fn test_translate() {
    let m = Matrix4::from_array([
            [0.840188, 0.911647, 0.277775, 0.364784],
            [0.394383, 0.197551, 0.55397, 0.513401],
            [0.783099, 0.335223, 0.477397, 0.95223],
            [0.79844, 0.76823, 0.628871, 1.0]]);
    let expected = Matrix4::from_array([
            [0.840188, 0.911647, 0.277775, 1.31426],
            [0.394383, 0.197551, 0.55397, 1.06311],
            [0.783099, 0.335223, 0.477397, 1.60706],
            [0.79844, 0.76823, 0.628871, 1.0]]);
    let answer = m.translate(0.141603, 0.717297, 0.635712);
    assert_matrix_eq(&answer, &expected);
}

#[test]
fn test_ortho() {
    let m = Matrix4::ortho(0.0, 640.0, 480.0, 0.0);
    assert_eq!(m[0][0], 0.003125);
    assert_f_eq(m[1][1], -0.0041666667);
    assert_eq!(m[2][2], -1.0);
    assert_eq!(m[3][3], 1.0);

    assert_eq!(m[3][0], -1.0);
    assert_eq!(m[3][1], 1.0);
    assert_eq!(m[3][2], 0.0);
}

#[cfg(test)]
fn assert_f_eq(left: f32, right: f32) {
    let diff = (left - right).abs();
    let within_range = diff < 0.0001;
    assert_eq!(within_range, true);
}

#[cfg(test)]
fn assert_matrix_eq(left: &Matrix4, right: &Matrix4) {
    assert_f_eq(left[0][0], right[0][0]);
    assert_f_eq(left[0][1], right[0][1]);
    assert_f_eq(left[0][2], right[0][2]);
    assert_f_eq(left[0][3], right[0][3]);

    assert_f_eq(left[1][0], right[1][0]);
    assert_f_eq(left[1][1], right[1][1]);
    assert_f_eq(left[1][2], right[1][2]);
    assert_f_eq(left[1][3], right[1][3]);

    assert_f_eq(left[2][0], right[2][0]);
    assert_f_eq(left[2][1], right[2][1]);
    assert_f_eq(left[2][2], right[2][2]);
    assert_f_eq(left[2][3], right[2][3]);

    assert_f_eq(left[3][0], right[3][0]);
    assert_f_eq(left[3][1], right[3][1]);
    assert_f_eq(left[3][2], right[3][2]);
    assert_f_eq(left[3][3], right[3][3]);
}

#[test]
fn test_mult() {
    let m1 = Matrix4::from_array([[0.635712 , 0.717297, 0.141603, 0.606969 ],
                                  [0.0163006, 0.242887, 0.137232, 0.804177 ],
                                  [0.156679 , 0.400944, 0.12979 , 0.108809 ],
                                  [0.998924 , 0.218257, 0.512932, 0.839112 ]]);
    let m2 = Matrix4::from_array([[0.840188 , 0.394383, 0.783099, 0.79844  ],
                                  [0.911647 , 0.197551, 0.335223, 0.76823  ],
                                  [0.277775 , 0.55397 , 0.477397, 0.628871 ],
                                  [0.364784 , 0.513401, 0.95223 , 0.916195 ]]);
    let answer = Matrix4::from_array([[1.44879 , 0.782479, 1.38385 , 1.70378 ],
                                      [0.566593, 0.543299, 0.925461, 1.02269 ],
                                      [0.572904, 0.268761, 0.422673, 0.614428],
                                      [1.48683 , 1.15203 , 1.89932 , 2.05661 ]]);
    let tmp = m1.mult(&m2);
    assert_matrix_eq(&tmp, &answer);
}
