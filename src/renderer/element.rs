use super::meshes::*;
use super::texture::Texture;
use super::texture_coords::TextureCoords;
use super::vao::Vao;
use crate::Renderer;
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub down_left_corner: (f32, f32),
    pub dimensions: (f32, f32),
}

impl Rectangle {
    pub fn get_height(&self) -> f32 {
        self.dimensions.1
    }
    pub fn get_width(&self) -> f32 {
        self.dimensions.0
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        (self.down_left_corner.0..self.down_left_corner.0 + self.dimensions.0).contains(&x)
            && (self.down_left_corner.1..self.down_left_corner.1 + self.dimensions.1).contains(&y)
    }
}

pub struct Element {
    pub vao: Vao,
    pub frame: Vao,
    pub rect: Rectangle,
    hover: bool,
    pub kind: ElementKind,
}

impl Element {
    pub fn plane(vao: Vao, frame: Vao, width: f32, height: f32) -> Self {
        Element {
            vao,
            frame,
            rect: Rectangle {
                dimensions: (width, height),
                down_left_corner: (-width / 2.0, -height / 2.0),
            },
            hover: false,
            kind: ElementKind::PLANE(Plane { areas: Vec::new() }),
        }
    }
    pub fn area(vao: Vao, frame: Vao, width: f32, height: f32) -> Self {
        Element {
            vao,
            frame,
            rect: Rectangle {
                dimensions: (width, height),
                down_left_corner: (-width / 2.0, -height / 2.0),
            },
            hover: false,
            kind: ElementKind::PLANE(Plane { areas: Vec::new() }),
        }
    }
}

pub enum ElementKind {
    PLANE(Plane),
    AREA(Area),
    POINT(Point),
}
impl Element {
    pub fn select(&self, x: f32, y: f32) -> Option<&Element> {
        if !self.rect.contains(x, y) {
            return None;
        }
        let element = match self.kind {
            ElementKind::PLANE(ref plane) => {
                let inside = plane
                    .areas
                    .iter()
                    .find(|ref area| area.select(x, y).is_some());
                inside.or(Some(self))
            }
            ElementKind::AREA(ref plane) => {
                let inside = plane
                    .points
                    .iter()
                    .find(|ref point| point.select(x, y).is_some());
                inside.or(Some(self))
            }
            ElementKind::POINT(ref _point) => Some(self),
        };
        element
    }
}

pub struct Plane {
    pub areas: Vec<Element>,
}

struct Area {
    points: Vec<Element>,
}

struct Point {
    value: f32,
}

#[test]
fn test() {
    let rect = Rectangle {
        down_left_corner: (-10., -10.),
        dimensions: (20., 20.),
    };
    // println!("{}", rect.contains(0., 0.));
    // println!("{}", rect.contains(9.8, 0.));
    // println!("{}", rect.contains(-19.0, 0.));
    // println!("{}", rect.contains(9.0, 10.));
    println!("{}", rect.contains(0.0, 11.));
}
