#![allow(unstable)]

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix4([[f32; 4]; 4]);

impl Matrix4 {
    pub fn as_array(&self) -> &[[f32; 4]; 4] {
        &self.0
    }

    pub fn from_array(array: [[f32; 4]; 4]) -> Self {
        Matrix4(array)
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
fn matrix_to_array() {
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
