use crate::quad::{Quad, QuadDirection};
use nalgebra::Rotation3;

const CUBE_TYPE_OFFSETS: &[(f32, f32)] = &[(1., 1.)];

#[derive(Copy, Clone)]
pub struct Cube {
    pub top_face: bool,
    pub bottom_face: bool,
    pub left_face: bool,
    pub right_face: bool,
    pub back_face: bool,
    pub front_face: bool,
    pub is_active: bool,
    pub cube_type: CubeType,
}

impl Cube {
    pub fn default() -> Self {
        Self {
            top_face: true,
            bottom_face: true,
            left_face: true,
            right_face: true,
            back_face: true,
            front_face: true,
            is_active: true,
            cube_type: CubeType::GRASS,
        }
    }

    pub fn new(
        top_face: bool,
        bottom_face: bool,
        left_face: bool,
        right_face: bool,
        back_face: bool,
        front_face: bool,
        cube_type: CubeType,
    ) -> Self {
        Self {
            top_face,
            bottom_face,
            left_face,
            right_face,
            back_face,
            front_face,
            is_active: true,
            cube_type,
        }
    }

    pub fn get_faces(&self, position: [f32; 3]) -> Vec<Quad> {
        let mut quads = Vec::new();
        let offset = CUBE_TYPE_OFFSETS[self.cube_type as usize];
        if self.back_face {
            quads.push(Quad::new(
                position,
                Rotation3::new([0., 0., 0.].into()),
                offset,
                QuadDirection::SIDE,
            ));
        }
        if self.front_face {
            let pitch: f32 = 180.;
            quads.push(Quad::new(
                position,
                Rotation3::from_euler_angles(0., pitch.to_radians(), 0.),
                offset,
                QuadDirection::SIDE,
            ))
        }
        if self.left_face {
            let pitch: f32 = 90.;
            quads.push(Quad::new(
                position,
                Rotation3::from_euler_angles(0., pitch.to_radians(), 0.),
                offset,
                QuadDirection::SIDE,
            ))
        }
        if self.right_face {
            let pitch: f32 = -90.;
            quads.push(Quad::new(
                position,
                Rotation3::from_euler_angles(0., pitch.to_radians(), 0.),
                offset,
                QuadDirection::SIDE,
            ))
        }
        if self.top_face {
            let roll: f32 = 90.;
            quads.push(Quad::new(
                position,
                Rotation3::from_euler_angles(roll.to_radians(), 0., 0.),
                offset,
                QuadDirection::UP,
            ))
        }
        if self.bottom_face {
            let roll: f32 = -90.;
            quads.push(Quad::new(
                position,
                Rotation3::from_euler_angles(roll.to_radians(), 0., 0.),
                offset,
                QuadDirection::DOWN,
            ))
        }
        return quads;
    }
}

#[derive(Copy, Clone)]
pub enum CubeType {
    GRASS,
    DIRT = 2,
    STONE = 3,
    WOOD = 4,
}
