use nalgebra::{Matrix4, Vector3};

pub struct Spatial {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    model_space_matrix: Matrix4<f32>,
    model_space_matrix_dirty: bool,
}

impl Spatial {
    pub fn new() -> Spatial {
        let translation = Vector3::new(0.0, 0.0, 0.0);
        let rotation = Vector3::new(0.0, 0.0, 0.0);
        let model_space_matrix = Matrix4::identity();
        let model_space_matrix_dirty = false;
        Spatial {
            translation,
            rotation,
            model_space_matrix,
            model_space_matrix_dirty,
        }
    }
    
    pub fn get_translation(&self) -> &Vector3<f32> {
        &self.translation
    }

    pub fn get_rotation(&self) -> &Vector3<f32> {
        &self.rotation
    }

    pub fn get_model_space_matrix(&mut self) -> &Matrix4<f32> {
        if (self.model_space_matrix_dirty) {
            // If the model space matrix is dirty (i.e. the rotation or
            // translation of the spatial has changed since the matrix
            // was last accessed) we need to rebuild it.

            let matrix_x_axis = Matrix4::from_axis_angle( 
                &Vector3::x_axis(), 
                self.rotation[0].to_radians() * -1.0);
            let matrix_y_axis = Matrix4::from_axis_angle( 
                &Vector3::y_axis(), 
                self.rotation[1].to_radians() * -1.0);
            let matrix_z_axis = Matrix4::from_axis_angle( 
                &Vector3::z_axis(), 
                self.rotation[2].to_radians() * -1.0);

            let matrix_translation = Matrix4::new_translation(
                &self.translation);

            self.model_space_matrix = Matrix4::identity();
            
            // The order in which the euler rotations are applied to the
            // spatial is important. Here the order gives what would be
            // expected from an FPS style game for the players view.
            self.model_space_matrix *= matrix_z_axis;
            self.model_space_matrix *= matrix_x_axis;
            self.model_space_matrix *= matrix_y_axis;

            // Apply the translation last.
            self.model_space_matrix *= matrix_translation;

            // The model space matrix is no longer in need of updating.
            self.model_space_matrix_dirty = false;
        }
        &self.model_space_matrix
    }

    pub fn set_translation(&mut self, translation: &Vector3<f32>) {
        self.translation = translation.clone_owned();
        self.model_space_matrix_dirty = true;
    }

    pub fn set_rotation(&mut self, rotation: &Vector3<f32>) {
        self.rotation = rotation.clone_owned();
        self.model_space_matrix_dirty = true;
    }
}
