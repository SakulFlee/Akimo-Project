use wgpu::{Device, Queue, TextureFormat};

use crate::{error::Error, resources::ModelDescriptor};

use super::{Material, Mesh};

pub struct Model {
    mesh: Mesh,
    material: Material,
}

impl Model {
    pub fn from_descriptor(
        descriptor: &ModelDescriptor,
        surface_format: &TextureFormat,
        device: &Device,
        queue: &Queue,
    ) -> Result<Self, Error> {
        let mesh = Mesh::from_descriptor(&descriptor.mesh_descriptor, device, queue);

        let material = Material::from_descriptor(
            &descriptor.material_descriptor,
            surface_format,
            device,
            queue,
        )?;

        Ok(Self { mesh, material })
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}