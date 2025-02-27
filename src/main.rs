//#![windows_subsystem = "windows"]

use winit::event_loop::{ControlFlow, EventLoop};

mod windowing;
mod ui; 
mod graphics;

fn main() {
    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(_) => return
    };
    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run_app(&mut windowing::App::default()).unwrap();
}