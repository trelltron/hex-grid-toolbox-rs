use std::ops::{Add, Mul};

// Tuple Helpers

pub fn offset_2d_tuple<T: Copy + Add<Output=T>>((x, y): (T, T), (off_x, off_y): (T, T)) -> (T, T) {
    (x + off_x, y + off_y)
}

pub fn scale_2d_tuple<T: Copy + Mul<Output=T>>((x, y): (T, T), factor: T) -> (T, T) {
    (x * factor, y * factor)
}

pub fn offset_3d_tuple<T: Copy + Add<Output=T>>((x, y, z): (T, T, T), (off_x, off_y, off_z): (T, T, T)) -> (T, T, T) {
    (x + off_x, y + off_y, z + off_z)
}

pub fn scale_3d_tuple<T: Copy + Mul<Output=T>>((x, y, z): (T, T, T), factor: T) -> (T, T, T) {
    (x * factor, y * factor, z * factor)
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Flat,
    Pointy
}

pub fn base_corner_coords(orient: Orientation) -> [(f32, f32); 6] {
    let s = 3f32.sqrt() / 2.0;
    match orient {
        Orientation::Flat => [
            (1.0, 0.0), (0.5, s), (-0.5, s), (-1.0, 0.0), (-0.5, -s), (0.5, -s)
        ],
        Orientation::Pointy => [
            (0.0, 1.0), (s, 0.5), (s, -0.5), (0.0, -1.0), (-s, -0.5), (-s, 0.5)
        ]
    }
}

pub fn corner_coords(offset: (f32, f32), shape: HexShape) -> [(f32, f32); 6] {
    let (orient, radius) = (shape.orient(), shape.radius());
    let r = base_corner_coords(orient);
    [
        offset_2d_tuple(scale_2d_tuple(r[0], radius), offset),
        offset_2d_tuple(scale_2d_tuple(r[1], radius), offset),
        offset_2d_tuple(scale_2d_tuple(r[2], radius), offset),
        offset_2d_tuple(scale_2d_tuple(r[3], radius), offset),
        offset_2d_tuple(scale_2d_tuple(r[4], radius), offset),
        offset_2d_tuple(scale_2d_tuple(r[5], radius), offset)
    ]
}

#[derive(Clone, Copy)]
pub enum HexShape {
    FlatTop(f32),
    PointyTop(f32)
}

impl HexShape {
    pub fn new(radius: f32, orient: Orientation) -> Self {
        match orient {
            Orientation::Flat => HexShape::FlatTop(radius),
            Orientation::Pointy => HexShape::PointyTop(radius)
        }
    }
    
    pub fn orient(&self) -> Orientation {
        match self {
            HexShape::FlatTop(_) => Orientation::Flat,
            HexShape::PointyTop(_) => Orientation::Pointy
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            HexShape::FlatTop(r) => *r,
            HexShape::PointyTop(r) => *r
        }
    }

    pub fn corners(&self, offset: (f32, f32)) -> [(f32, f32); 6] {
        corner_coords(offset, *self)
    }
}