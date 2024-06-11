use akimo_runtime::{
    app::{AppRuntime, RuntimeSettings},
    log,
    winit::{error::EventLoopError, event_loop::EventLoop},
};

use crate::app::ClearScreenApp;

pub fn entrypoint(event_loop_result: Result<EventLoop<()>, EventLoopError>) {
    log::init();

    let event_loop = event_loop_result.expect("Event Loop failure");
    let settings = RuntimeSettings::default();

    AppRuntime::<ClearScreenApp>::liftoff(event_loop, settings).expect("Runtime failure");
}
