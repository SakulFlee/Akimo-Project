use akimo_runtime::winit::event_loop::EventLoop;
use app_example_render_server_triangle::entrypoint::entrypoint;

fn main() {
    let event_loop = EventLoop::builder().build();

    entrypoint(event_loop);
}
