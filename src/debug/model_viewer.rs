use crate::render::render::{Mesh, RenderData};
use dolly::prelude::{Arm, Position, RightHanded, YawPitch};
use dolly::rig::CameraRig;
use glam::Vec3;
use mint;

pub struct ModelViewer {
    camera_rig: CameraRig<RightHanded>,
    mesh: Mesh,

    // Camera control parameters
    orbit_sensitivity: f32,
    pan_speed: f32,
    zoom_speed: f32,
    zoom_min: f32,
    zoom_max: f32,
}

impl Default for ModelViewer {
    fn default() -> Self {
        let initial_distance = 3.0;
        let camera: CameraRig = CameraRig::builder()
            .with(Position::new(glam::Vec3::ZERO))
            .with(YawPitch::new())
            .with(Arm::new(Vec3::Z * initial_distance))
            .build();

        // dummy mesh (triangle)
        let mesh = Mesh {
            vertices: vec![[0.0, 0.5, 0.0], [-0.8, -0.5, 0.0], [0.5, -0.5, 0.0]],
            indices: vec![0, 1, 2],
        };

        Self {
            camera_rig: camera,
            mesh,
            orbit_sensitivity: 0.3,
            pan_speed: 0.001,
            zoom_speed: 0.01,
            zoom_min: 1.0,
            zoom_max: 10.0,
        }
    }
}

impl ModelViewer {
    fn load_model(&mut self) {}
}

impl ModelViewer {
    pub fn handle_input(&mut self, response: &egui::Response, ui: &egui::Ui) {
        // Orbit - left mouse drag to rotate
        if response.dragged_by(egui::PointerButton::Primary) {
            let delta = response.drag_delta();
            self.camera_rig.driver_mut::<YawPitch>().rotate_yaw_pitch(
                -delta.x * self.orbit_sensitivity,
                -delta.y * self.orbit_sensitivity,
            );
        }

        // Pan - right mouse drag
        if response.dragged_by(egui::PointerButton::Secondary) {
            let delta = response.drag_delta();
            let transform = &self.camera_rig.final_transform;
            let right: mint::Vector3<f32> = transform.right();
            let up: mint::Vector3<f32> = transform.up();

            let right_vec = Vec3::new(right.x, right.y, right.z);
            let up_vec = Vec3::new(up.x, up.y, up.z);

            let pan_offset =
                -delta.x * self.pan_speed * right_vec + delta.y * self.pan_speed * up_vec;

            let pos = self.camera_rig.driver_mut::<Position>();
            let current: mint::Point3<f32> = pos.position;
            let current_vec = Vec3::new(current.x, current.y, current.z);
            let new_pos = current_vec + pan_offset;

            pos.position = mint::Point3 {
                x: new_pos.x,
                y: new_pos.y,
                z: new_pos.z,
            };
        }

        // Zoom - scroll wheel
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll != 0.0 {
            let arm = self.camera_rig.driver_mut::<Arm>();
            let offset: mint::Vector3<f32> = arm.offset;
            let offset_vec = Vec3::new(offset.x, offset.y, offset.z);

            let new_distance = (offset_vec.length() - scroll * self.zoom_speed)
                .clamp(self.zoom_min, self.zoom_max);

            let new_offset = offset_vec.normalize() * new_distance;
            arm.offset = mint::Vector3 {
                x: new_offset.x,
                y: new_offset.y,
                z: new_offset.z,
            };
        }

        self.camera_rig.update(0.016);
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::Window::new("Model Viewer").show(ctx, |ui| if ui.button("Load Model").clicked() {});

        // allocate space for renderer
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
        self.handle_input(&response, ui);

        // build callback
        let callback = egui_wgpu::Callback::new_paint_callback(
            rect,
            RenderData {
                mesh: self.mesh.clone(),
                aspect: rect.width() / rect.height(),
                camera_transform: self.camera_rig.final_transform,
            },
        );
        ui.painter().add(callback);
    }
}
