use glam::{vec4, Vec3};

use crate::gpu;

#[derive(Clone, Debug)]
pub enum Light {
    Point {
        position: Vec3,
        radius: f32,
        color: Vec3,
        range: f32,
    },

    Spot {
        position: Vec3,
        radius: f32,
        color: Vec3,
        range: f32,
        direction: Vec3,
        angle: f32,
    },
}

impl Light {
    pub(crate) fn serialize(&self) -> gpu::Light {
        let d0;
        let d1;
        let d2;

        match self {
            Light::Point {
                position,
                radius,
                color,
                range,
            } => {
                d0 = position.extend(*radius);
                d1 = color.extend(*range);

                d2 = vec4(
                    f32::from_bits(gpu::Light::TYPE_POINT),
                    Default::default(),
                    Default::default(),
                    Default::default(),
                );
            }

            Light::Spot {
                position,
                radius,
                color,
                range,
                direction,
                angle,
            } => {
                let direction = gpu::Normal::encode(*direction);

                d0 = position.extend(*radius);
                d1 = color.extend(*range);

                d2 = vec4(
                    f32::from_bits(gpu::Light::TYPE_SPOT),
                    direction.x,
                    direction.y,
                    *angle,
                );
            }
        }

        gpu::Light {
            d0,
            d1,
            d2,
            d3: Default::default(),
            prev_d0: Default::default(),
            prev_d1: Default::default(),
            prev_d2: Default::default(),
        }
    }
}
