use super::super::alg::{utils, Mat4, Vec4};

pub struct Camera {
    position: Vec4,
    pitch: f32,
    yaw: f32,
    roll: f32,
    zoom: f32, // used in orthographic projections
}

impl Camera {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Camera {
            position: Vec4([0., 0., 0., 1.]),
            pitch: 0.,
            yaw: 0.,
            roll: 0.,
            zoom: 300.0,
        }
    }
    #[allow(dead_code)]
    pub fn new(position: Vec4, pitch: f32, yaw: f32, roll: f32, zoom: f32) -> Self {
        Camera {
            position,
            pitch,
            yaw,
            roll,
            zoom,
        }
    }
    #[allow(dead_code)]
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }
    #[allow(dead_code)]
    pub fn get_position(&self) -> Vec4 {
        self.position
    }
    #[allow(dead_code)]
    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }
    #[allow(dead_code)]
    pub fn get_yaw(&self) -> f32 {
        self.yaw
    }
    #[allow(dead_code)]
    fn increment_zoom(&mut self, increment: f32) {
        self.zoom *= increment;
    }
    #[allow(dead_code)]
    fn increment_pitch(&mut self, increment: f32) {
        self.pitch += increment;
    }
    #[allow(dead_code)]
    fn increment_yaw(&mut self, increment: f32) {
        self.yaw += increment;
    }
    #[allow(dead_code)]
    pub fn get_roll(&self) -> f32 {
        self.roll
    }
    #[allow(dead_code)]
    pub fn set_position(&mut self, position: Vec4) {
        self.position = position;
    }
    fn increment_position(&mut self, position: Vec4) {
        self.position = self.position + position;
    }
    #[allow(dead_code)]
    pub fn mouse_move_2d(&mut self, _dx: f32, dy: f32) {
        let increment = (1.05 as f32).powf(-dy * 0.5);
        self.increment_zoom(increment);
    }
    #[allow(dead_code)]
    pub fn mouse_move_3d(&mut self, dx: f32, dy: f32) {
        self.increment_pitch(dy * -0.1);
        self.increment_yaw(dx as f32 * 0.1);
    }
    #[allow(dead_code)]
    pub fn key_move_2d(&mut self, key: &str) {
        let increment_position: Vec4 = match key {
            "w" => Vec4([0.0, 1.0, 0.0, 0.0]),
            "s" => Vec4([0.0, -1.0, 0.0, 0.0]),
            "a" => Vec4([-1.0, 0.0, 0.0, 0.0]),
            "d" => Vec4([1.0, 0.0, 0.0, 0.0]),
            _ => Vec4::new(),
        };
        self.increment_position(increment_position.mul(self.zoom * 0.05));
    }
    #[allow(dead_code)]
    pub fn key_move_3d(&mut self, key: &str) {
        let mut increment_position = Vec4::new();
        let cos_pitch = utils::to_radians(self.pitch).cos();
        match key {
            "w" => {
                increment_position[2] = utils::to_radians(self.yaw).cos() * cos_pitch;
                increment_position[0] = utils::to_radians(self.yaw).sin() * cos_pitch;
                increment_position[1] = utils::to_radians(self.pitch).sin();
            }
            "s" => {
                increment_position[2] = -utils::to_radians(self.yaw).cos() * cos_pitch;
                increment_position[0] = -utils::to_radians(self.yaw).sin() * cos_pitch;
                increment_position[1] = -utils::to_radians(self.pitch).sin();
            }
            "d" => {
                increment_position[2] = utils::to_radians(self.yaw).sin();
                increment_position[0] = -utils::to_radians(self.yaw).cos();
            }
            "a" => {
                increment_position[2] = -utils::to_radians(self.yaw).sin();
                increment_position[0] = utils::to_radians(self.yaw).cos();
            }
            _ => {}
        }
        self.increment_position(increment_position.mul(-0.1));
    }
    #[allow(dead_code)]
    pub fn get_view_matrix(&self) -> Mat4 {
        utils::view_matrix(&self.position, self.pitch, self.yaw, self.roll)
    }
}
