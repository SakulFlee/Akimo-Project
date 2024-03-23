use std::sync::Arc;

use log::info;
use wgpu::{rwh::HasWindowHandle, Surface, SurfaceConfiguration};
use winit::{
    dpi::{PhysicalSize, Size},
    window::Window,
};

use crate::context::Context;

pub struct SurfaceWrapper {
    surface: Option<Surface<'static>>,
    configuration: Option<SurfaceConfiguration>,
}

impl SurfaceWrapper {
    pub fn new() -> Self {
        Self {
            surface: None,
            configuration: None,
        }
    }

    pub fn get(&self) -> Option<&Surface> {
        self.surface.as_ref()
    }

    pub fn configuration(&self) -> &SurfaceConfiguration {
        self.configuration.as_ref().unwrap()
    }

    pub fn resume(&mut self, context: &Context, window: Arc<Window>) {
        let window_size = window.inner_size();
        info!("Surface resume (size: {:?})", window_size);

        self.surface = Some(context.instance().create_surface(window).unwrap());

        let surface = self.surface.as_ref().unwrap();

        let mut configuration = surface
            .get_default_config(&context.adapter(), window_size.width, window_size.height)
            .expect("Surface isn't supported by adapter");

        // Add SRGB (view) format
        configuration
            .view_formats
            .push(configuration.format.add_srgb_suffix());

        surface.configure(context.device(), &configuration);
        self.configuration = Some(configuration);
    }

    pub fn resize<S: Into<PhysicalSize<u32>>>(&mut self, context: &Context, size: S) {
        let configuration = self.configuration.as_mut().unwrap();

        let size = size.into();
        configuration.width = size.width;
        configuration.height = size.height;

        let surface = self.surface.as_ref().unwrap();
        surface.configure(&context.device(), &configuration);
    }
}