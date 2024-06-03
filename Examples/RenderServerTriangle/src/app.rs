use akimo_runtime::{
    nalgebra::{Vector2, Vector3},
    resources::{MaterialDescriptor, MeshDescriptor, ModelDescriptor, TextureDescriptor, Vertex},
    runtime::App,
    server::RenderServer,
    wgpu::{Device, Queue, SurfaceConfiguration, TextureView},
};

pub struct RenderServerTriangleApp {
    render_server: RenderServer,
}

impl App for RenderServerTriangleApp {
    fn init(config: &SurfaceConfiguration, _device: &Device, _queue: &Queue) -> Self
    where
        Self: Sized,
    {
        let vertices = vec![
            Vertex {
                position_coordinates: Vector3::new(-1.0, -1.0, 0.0),
                texture_coordinates: Vector2::new(0.0, 0.0),
            },
            Vertex {
                position_coordinates: Vector3::new(1.0, -1.0, 0.0),
                texture_coordinates: Vector2::new(0.0, 0.0),
            },
            Vertex {
                position_coordinates: Vector3::new(0.0, 1.0, 0.0),
                texture_coordinates: Vector2::new(0.0, 0.0),
            },
        ];
        let indices = vec![0, 1, 2];

        let mut render_server = RenderServer::new(config.format);

        render_server.spawn_model(ModelDescriptor::FromDescriptors(
            MeshDescriptor { vertices, indices },
            MaterialDescriptor::PBRCustomShader(TextureDescriptor::EMPTY, include_str!("rgb.wgsl")),
        ));

        Self { render_server }
    }

    fn update(&mut self) {}

    fn render(&mut self, view: &TextureView, device: &Device, queue: &Queue) {
        self.render_server.render(view, device, queue);
    }
}
