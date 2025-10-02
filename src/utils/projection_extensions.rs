use bevy::prelude::{OrthographicProjection, Projection};

pub trait ProjectionExt {
    fn as_orthographic(&self) -> Option<&OrthographicProjection>;

    fn as_orthographic_mut(&mut self) -> Option<&mut OrthographicProjection>;
}

impl ProjectionExt for Projection {
    fn as_orthographic(&self) -> Option<&OrthographicProjection> {
        match self {
            Projection::Orthographic(orthographic) => Some(orthographic),
            _ => None,
        }
    }

    fn as_orthographic_mut(&mut self) -> Option<&mut OrthographicProjection> {
        match self {
            Projection::Orthographic(orthographic) => Some(orthographic),
            _ => None,
        }
    }
}
