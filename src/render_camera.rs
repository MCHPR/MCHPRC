use crate::spatial::Spatial;
use nalgebra::{Matrix4, Vector3};

pub struct Camera {
    spatial: Spatial,
    aspect_ratio: f32,
    vfov: f32,
    mat_projection: Matrix4<f32>,
    mat_projection_dirty: bool,
}

impl Camera {
    pub fn new(aspect_ratio: f32, vfov: f32) -> Camera {
        let spatial = Spatial::new();
        let mat_projection = Matrix4::identity();
        Camera {
            spatial,
            aspect_ratio,
            vfov,
            mat_projection,
            mat_projection_dirty: true,
        }
    }

    pub fn borrow_spatial(&self) -> &Spatial {
        &self.spatial
    }

    pub fn borrow_spatial_mut(&mut self) -> &mut Spatial {
        &mut self.spatial
    }

    pub fn get_projection(&mut self) -> &Matrix4<f32> {
        if (self.mat_projection_dirty) {
            self.rebuild_projection_mat();
            self.mat_projection_dirty = false;
        }
        &self.mat_projection
    }

    fn rebuild_projection_mat(&mut self) {
        self.mat_projection = Matrix4::new_perspective(self.aspect_ratio, 
            self.vfov, 0.1, 4096.0);
    }
}
