use super::mat4::Mat4;
use super::vec4::Vec4;

// pi : 180 = rad : degree
#[allow(dead_code)]
pub fn to_radians(degree: f32) -> f32 {
    degree * std::f64::consts::PI as f32 / 180.
}

#[allow(dead_code)]
pub fn approx_equal(a: f32, b: f32, dp: u8) -> bool {
    let p = 10f32.powi(-(dp as i32));
    (a - b).abs() < p
}

#[allow(dead_code)]
pub fn orthographic_matrix(aspect_ratio: f32, width: f32, near: f32, far: f32) -> Mat4 {
    let height = width / aspect_ratio;
    let mut translation = Mat4::identity().translate(&Vec4([0.0, 0.0, -(far + near) / 2.0, 1.0]));
    translation[10] = -1.0;
    let scale = Mat4::identity().scale(&Vec4([1.0 / width, 1.0 / height, 2.0 / (far - near), 1.0]));
    scale * translation
}

#[allow(dead_code)]
pub fn perspective_matrix(aspect_ratio: f32, fov: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / to_radians(fov * 0.5).tan();
    let range_inv = 1.0 / (near - far);

    let mut m = Mat4::identity();
    m[0] = f / aspect_ratio;
    m[5] = f;
    m[10] = (far + near) * range_inv;
    m[11] = 2.0 * near * far * range_inv;
    m[14] = -1.;
    m[15] = 0.;
    m
}

#[allow(dead_code)]
pub fn view_matrix(position: &Vec4, pitch: f32, yaw: f32, _roll: f32) -> Mat4 {
    let m = Mat4::identity();
    let m = m.rotate_x(to_radians(pitch)).rotate_y(to_radians(yaw));
    let m = m.translate(&-*position);
    m
}
