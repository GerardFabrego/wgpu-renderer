use std::f32::consts::FRAC_PI_2;

use cgmath::{vec3, InnerSpace, Rad, Vector3};

use super::Camera;

pub struct CameraController {
    pub speed: f32,
    pub rotation_speed: f32,
}

const UP: Vector3<f32> = vec3(0.0, 1.0, 0.0);
const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

impl CameraController {
    pub fn move_forward(&self, camera: &mut Camera) {
        let (_, forward) = get_local_axis(camera);
        camera.position += forward * self.speed;
    }

    pub fn move_backwards(&self, camera: &mut Camera) {
        let (_, forward) = get_local_axis(camera);
        camera.position -= forward * self.speed;
    }

    pub fn move_right(&self, camera: &mut Camera) {
        let (right, _) = get_local_axis(camera);
        camera.position += right * self.speed;
    }

    pub fn move_left(&self, camera: &mut Camera) {
        let (right, _) = get_local_axis(camera);
        camera.position -= right * self.speed;
    }
    pub fn move_up(&self, camera: &mut Camera) {
        camera.position += UP * self.speed;
    }
    pub fn move_down(&self, camera: &mut Camera) {
        camera.position -= UP * self.speed;
    }

    pub fn rotate(&self, camera: &mut Camera, (yaw, pitch): (f32, f32)) {
        camera.yaw += Rad(yaw * self.rotation_speed);
        camera.pitch += Rad(pitch * self.rotation_speed);

        if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
            camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
    }
}

fn get_local_axis(camera: &Camera) -> (Vector3<f32>, Vector3<f32>) {
    let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
    let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
    let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();

    (right, forward)
}
