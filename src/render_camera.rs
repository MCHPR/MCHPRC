use nalgebra::{Matrix4, Vector3};

pub struct Camera {
    projection: Matrix4<f32>,
    world_space: Matrix4<f32>,
    // TODO There is no actual reason besides lazyness to store transform
    // or rotation, because both values can be extracted from the world
    // space matrix. it was 4:26am what do you expect?
    world_translate: Vector3<f32>,
    world_rotation: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        let projection = Matrix4::identity();
        let world_space = Matrix4::identity();
        let world_translate = Vector3::new(0.0, 0.0, 0.0);
        let world_rotation = Vector3::new(0.0, 0.0, 0.0);
        Camera {
            projection,
            world_space,
            world_translate,
            world_rotation,
        }
    }

    pub fn get_projection(&self) -> Matrix4<f32> {
        self.projection.clone_owned()
    }

    pub fn get_translate(&self) -> Vector3<f32> {
        self.world_translate.clone_owned()
    }

    pub fn set_translate(&mut self, translate: &Vector3<f32>) {
        self.world_translate = translate.clone_owned();
        self.rebuild_world_space();
    }

    pub fn get_rotation(&self) -> Vector3<f32> {
        self.world_rotation.clone_owned()
    }

    pub fn set_rotation(&mut self, rotation: &Vector3<f32>) {
        self.world_rotation = rotation.clone_owned();
        self.rebuild_world_space();
    }

    pub fn get_world_space(&self) -> Matrix4<f32> {
        self.world_space.clone_owned()
    }

    pub fn build_frustum(&mut self, w: u32, h: u32, vfov: f32) {
        let w = w as f32;
        let h = h as f32;
        // NAlgebra includes a perspective matrix tool, I love it.
        self.projection = Matrix4::new_perspective(w / h, vfov.to_radians(), 0.1, 4086.0);
    }

    fn rebuild_world_space(&mut self) {
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
            self.world_rotation[0].to_radians() * -1.0,
        );
        let y_rot_mat = Matrix4::from_axis_angle(
            &Vector3::y_axis(),
            self.world_rotation[1].to_radians() * -1.0,
        );
        let z_rot_mat = Matrix4::from_axis_angle(
            &Vector3::z_axis(),
            self.world_rotation[2].to_radians() * -1.0,
        );
        let trans_mat = Matrix4::new_translation(&(self.world_translate * -1.0));

        self.world_space = Matrix4::identity();
        // Remember that while you can reach any rotation with euler
        // axes, the order that those rotations apply drastically affect
        // the final rotation.

        // As such it is important to first roll the camera (unlikely to
        // be used,) then pitch the camera, then yaw the camera. These are
        // the z, x, and y axes respectively. This gives the expected
        // behavior for the camera.
        self.world_space *= z_rot_mat;
        self.world_space *= x_rot_mat;
        self.world_space *= y_rot_mat;

        // We translate last, so that the translation is still along
        // the original world axes.
        self.world_space *= trans_mat;
    }
}
