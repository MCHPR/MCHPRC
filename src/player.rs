use nalgebra::{Matrix4, Vector3, Rotation3};
use crate::spatial::Spatial;

pub struct Player {
	pub spatial: Spatial,
    pub velocity: Vector3<f32>,

    pub control_heading: Vector3<f32>,
    pub control_vector: Vector3<f32>,
}

impl Player {
	pub fn new() -> Player {
		let spatial = Spatial::new();
        let velocity = Vector3::new(0.0, 0.0, 0.0);
        let control_heading = Vector3::new(0.0, 0.0, 0.0);
        let control_vector = Vector3::new(0.0, 0.0, 0.0);
		Player {
			spatial,
            velocity,
            control_heading,
            control_vector,
		}
	}

    pub fn update(&mut self) {
        //Apply our control heading and vector
        self.velocity = self.control_vector.clone_owned();
        self.velocity = Rotation3::from_axis_angle(
            &Vector3::y_axis(),
            self.control_heading[1])
            * self.velocity;
        if self.velocity.magnitude() > 0.001 {
            self.velocity = self.velocity.normalize();
        }

        self.velocity *= 0.2;

        self.move_player(&self.velocity.clone_owned());
    }

    pub fn move_player(&mut self, vector: &Vector3<f32>) {
        //This function will later be used to process physics as well.
        let mut translation = self.borrow_spatial().get_translation()
            .clone_owned();
        translation += vector;
        self.borrow_spatial_mut().set_translation(&translation);
    }

    pub fn borrow_spatial(&self) -> &Spatial {
        &self.spatial
    }

    pub fn borrow_spatial_mut(&mut self) -> &mut Spatial {
        &mut self.spatial
    }

    pub fn get_velocity(&self) -> &Vector3<f32> {
        &self.velocity
    }

    pub fn get_control_heading(&self) -> &Vector3<f32> {
        &self.control_heading
    }

    pub fn get_control_vector(&self) -> &Vector3<f32> {
        &self.control_vector
    }

    pub fn set_velocity(&mut self, velocity: &Vector3<f32>) {
        self.velocity = velocity.clone_owned();
    }
    
    pub fn set_control_heading(&mut self, heading: &Vector3<f32>) {
        self.control_heading = heading.clone_owned();
    }

    pub fn set_control_vector(&mut self, vector: &Vector3<f32>) {
        self.control_vector = vector.clone_owned();
    }
}
