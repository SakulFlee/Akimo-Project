use wgpu::CreateSurfaceError;
use winit::error::EventLoopError;

#[derive(Debug)]
pub enum Error {
    EntityExistsAlready,
    NoAdapters,
    RequestDeviceError,
    NoMatch,
    SurfaceError(CreateSurfaceError),
    EventLoopError(EventLoopError),
    MutexPoisonError(String),
}
