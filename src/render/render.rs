use bytemuck;
use dolly;
use dolly::prelude::RightHanded;
use glam::{Mat4, Vec3};
use wgpu;
use wgpu::util::DeviceExt;

const SHADER_SOURCE: &str = include_str!("../shaders/shader.wgsl");

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniforms {
    view_proj: [[f32; 4]; 4], // 4x4 matrix
}

/// Data needed for rendering callback
pub struct RenderData {
    pub mesh: Mesh,
    pub aspect: f32,
    pub camera_transform: dolly::transform::Transform<RightHanded>,
}

impl RenderData {
    fn compute_view_proj(&self) -> Mat4 {
        // Create projection matrix
        let projection = Mat4::perspective_rh(45.0_f32.to_radians(), self.aspect, 0.1, 100.0);

        // Get view matrix from dolly's camera transform
        let rot = glam::Quat::from(self.camera_transform.rotation);
        let pos = glam::Vec3::from(self.camera_transform.position);
        let view = Mat4::from_rotation_translation(rot, pos).inverse();

        projection * view
    }
}

/// GPU state for rendering
struct GpuState {
    pipeline: wgpu::RenderPipeline,
    // mesh data
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    // camera data
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
}

impl GpuState {
    fn new(device: &wgpu::Device, mesh: &Mesh) -> Self {
        // define triangle (3D positions)

        let vertices = mesh
            .vertices
            .iter()
            .map(|&pos| Vertex { position: pos })
            .collect::<Vec<_>>();

        let indices = mesh.indices.iter().copied().collect::<Vec<_>>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create camera uniform buffer
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: std::mem::size_of::<CameraUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout for camera
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Create pipeline layout with bind group
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        // create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Triangle Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
        });

        // Create bind group for camera
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // create pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        GpuState {
            pipeline,
            vertex_buffer,
            index_buffer,
            camera_buffer,
            camera_bind_group,
            num_indices: indices.len() as u32,
        }
    }
}

impl egui_wgpu::CallbackTrait for RenderData {
    // Build render pipeline
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        // build gpu state if it does not exist
        if callback_resources.get::<GpuState>().is_none() {
            let gpu_state = GpuState::new(device, &self.mesh);
            callback_resources.insert(gpu_state);
        }
        let resources = callback_resources.get::<GpuState>().unwrap();

        let view_proj = self.compute_view_proj();

        let camera_uniforms = CameraUniforms {
            view_proj: view_proj.to_cols_array_2d(),
        };

        queue.write_buffer(
            &resources.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniforms]),
        );

        Vec::new()
    }

    fn paint(
        &self,
        info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        // get resources
        let resources: &GpuState = callback_resources.get().unwrap();

        // set pipeline
        render_pass.set_pipeline(&resources.pipeline);

        // set bind group
        render_pass.set_bind_group(0, &resources.camera_bind_group, &[]);

        // set vert/index buffer
        render_pass.set_vertex_buffer(0, resources.vertex_buffer.slice(..));
        render_pass.set_index_buffer(resources.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        // draw indexed
        render_pass.draw_indexed(0..resources.num_indices, 0, 0..1);
    }
}
