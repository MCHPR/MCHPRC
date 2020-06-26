use nalgebra::{Matrix4, Vector3};

pub struct Camera {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    aspect_ratio: f32,
    vfov: f32,
    mat_projection: Matrix4<f32>,
    mat_world_space: Matrix4<f32>,
    mat_projection_dirty: bool,
    mat_world_space_dirty: bool,
}

impl Camera {
    pub fn new(aspect_ratio: f32, vfov: f32) -> Camera {
        let translation = Vector3::new(0.0, 0.0, 0.0);
        let rotation = Vector3::new(0.0, 0.0, 0.0);
        let mat_projection = Matrix4::identity();
        let mat_world_space = Matrix4::identity();
        Camera {
            translation,
            rotation,
            aspect_ratio,
            vfov,
            mat_projection,
            mat_world_space,
            mat_projection_dirty: true,
            mat_world_space_dirty: true,
        }
    }

    pub fn get_translation(&self) -> &Vector3<f32> {
        &self.translation
    }

    pub fn get_rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }

    pub fn get_projection(&mut self) -> &Matrix4<f32> {
        if (self.mat_projection_dirty) {
            self.rebuild_projection_mat();
            self.mat_projection_dirty = false;
        }
        &self.mat_projection
    }

    pub fn get_world_space(&mut self) -> &Matrix4<f32> {
        if (self.mat_world_space_dirty) {
            self.rebuild_world_space_mat();
            self.mat_world_space_dirty = false;
        }
        &self.mat_world_space
    }

    pub fn set_translation(&mut self, translation: &Vector3<f32>) {
        self.translation = translation.clone_owned();
        self.mat_world_space_dirty = true;
    }

    pub fn set_rotation(&mut self, rotation: &Vector3<f32>) {
        self.rotation = rotation.clone_owned();
        self.mat_world_space_dirty = true;
    }

    fn rebuild_projection_mat(&mut self) {
        self.mat_projection = Matrix4::new_perspective(self.aspect_ratio, 
            self.vfov, 0.1, 4096.0);
    }

    fn rebuild_world_space_mat(&mut self) {
        // For now, rebuilding the entire world space matrix when the
        // camera changes position or rotation is necessary. Later it
        // should be possible to update only the relevant portion of the
        // world space matrix.

        // We apply the opposite translation and rotation to the world
        // space matrix. This is because we can not move the camera, but
        // must instead move everything else around the camera. For
        // instance if our camera moves right, everything appears to move
        // left.
        let x_rot_mat = Matrix4::from_axis_angle(
            &Vector3::x_axis(),
            self.rotation[0].to_radians() * -1.0,
        );
        let y_rot_mat = Matrix4::from_axis_angle(
            &Vector3::y_axis(),
            self.rotation[1].to_radians() * -1.0,
        );
        let z_rot_mat = Matrix4::from_axis_angle(
            &Vector3::z_axis(),
            self.rotation[2].to_radians() * -1.0,
        );
        let trans_mat = Matrix4::new_translation(&self.translation);

        self.mat_world_space = Matrix4::identity();
        // Remember that while you can reach any rotation with euler
        // axes, the order that those rotations apply drastically affect
        // the final rotation.

        // As such it is important to first roll the camera (unlikely to
        // be used,) then pitch the camera, then yaw the camera. These are
        // the z, x, and y axes respectively. This gives the expected
        // behavior for the camera.
        self.mat_world_space *= z_rot_mat;
        self.mat_world_space *= x_rot_mat;
        self.mat_world_space *= y_rot_mat;

        // We translate last, so that the translation is still along
        // the original world axes.
        self.mat_world_space *= trans_mat;
    }
}
